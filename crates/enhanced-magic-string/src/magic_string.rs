use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use crate::{error::Result, utils::common::get_relative_path};
use farmfe_utils::relative;
use parking_lot::Mutex;
use sourcemap::{SourceMap, SourceMapBuilder};

use crate::{
  chunk::Chunk,
  mappings::Mappings,
  types::SourceMapOptions,
  utils::{char_string::CharString, get_locator::get_locator},
};

pub type ExclusionRange = (usize, usize);

#[derive(Default)]
pub struct MagicStringOptions {
  pub filename: Option<String>,
  pub indent_exclusion_ranges: Vec<ExclusionRange>,
  pub ignore_list: Vec<CharString>,
  pub source_map_chain: Vec<Arc<String>>,
}

pub struct MagicString {
  pub original: CharString,
  pub outro: CharString,
  pub intro: CharString,

  pub first_chunk: Arc<Mutex<Chunk>>,
  pub last_chunk: Arc<Mutex<Chunk>>,
  pub last_searched_chunk: Arc<Mutex<Chunk>>,
  pub chunk_by_start: HashMap<usize, Arc<Mutex<Chunk>>>,
  pub chunk_by_end: HashMap<usize, Arc<Mutex<Chunk>>>,

  pub filename: Option<String>,
  pub indent_exclusion_ranges: Vec<ExclusionRange>,
  pub sourcemap_locations: HashSet<usize>,
  pub stored_names: HashMap<CharString, bool>,
  pub indent_str: Option<CharString>,
  pub ignore_list: Vec<CharString>,
  source_map_chain: Vec<Arc<String>>,

  pub separator: char,
}

impl MagicString {
  pub fn new(original: &str, options: Option<MagicStringOptions>) -> Self {
    let options = options.unwrap_or_default();
    let original = CharString::new(original);
    let chunk = Arc::new(Mutex::new(Chunk::new(0, original.len(), original.clone())));

    let mut magic_string = Self {
      original: original.clone(),
      outro: CharString::new(""),
      intro: CharString::new(""),
      first_chunk: chunk.clone(),
      last_chunk: chunk.clone(),
      last_searched_chunk: chunk,
      chunk_by_start: HashMap::new(),
      chunk_by_end: HashMap::new(),
      filename: options.filename,
      indent_exclusion_ranges: options.indent_exclusion_ranges,
      sourcemap_locations: HashSet::new(),
      stored_names: HashMap::new(),
      indent_str: None,
      ignore_list: options.ignore_list,
      source_map_chain: options.source_map_chain,
      separator: '\n',
    };

    magic_string
      .chunk_by_start
      .insert(0, magic_string.first_chunk.clone());
    magic_string
      .chunk_by_end
      .insert(0, magic_string.last_chunk.clone());

    magic_string
  }

  pub fn get_source_map_chain(&self) -> Vec<SourceMap> {
    let mut chain = self
      .source_map_chain
      .iter()
      .map(|source| SourceMap::from_slice(source.as_bytes()).unwrap())
      .filter(|source| {
        // if the source map is empty, we should ignore it
        source.get_token_count() > 0
      })
      .collect::<Vec<_>>();
    chain.reverse();

    chain
  }

  pub fn generate_map(&self, opts: SourceMapOptions) -> Result<SourceMap> {
    let source_index = 0;
    // let names: Vec<&CharString> = self.stored_names.keys().collect();

    let locate = get_locator(&self.original);
    let mut mappings = Mappings::new(opts.hires.unwrap_or_default());

    if !self.intro.is_empty() {
      mappings.advance(&self.intro);
    }

    self.first_chunk.lock().each_next(|chunk| {
      let loc = locate(chunk.start);

      if !chunk.intro.is_empty() {
        mappings.advance(&chunk.intro);
      }

      if !chunk.edited {
        mappings.add_unedited_chunk(
          source_index,
          &chunk,
          &self.original,
          loc,
          &self.sourcemap_locations,
        )
      } else {
        unimplemented!("chunk.edited")
      }

      if !chunk.outro.is_empty() {
        mappings.advance(&chunk.outro)
      }
    });

    let source = if let Some(src) = &opts.source {
      get_relative_path(opts.file.clone().unwrap_or_default().as_str(), src).unwrap()
    } else {
      opts.file.clone().unwrap_or_default()
    };

    let mut sourcemap_builder = SourceMapBuilder::new(opts.file.as_ref().map(|f| f.as_str()));
    let src_id = sourcemap_builder.add_source(&source);

    let inline_content = opts.include_content.unwrap_or(false);

    let contet = if inline_content {
      Some(self.original.to_string())
    } else {
      None
    };
    sourcemap_builder.set_source_contents(src_id, contet.as_deref());
    mappings.into_sourcemap_mappings(&mut sourcemap_builder);
    Ok(sourcemap_builder.into_sourcemap())
  }

  pub fn prepend(&mut self, str: &str) {
    let mut new_intro = CharString::new(str);
    new_intro.append(&self.intro);
    self.intro = new_intro;
  }

  pub fn append(&mut self, str: &str) {
    let mut new_outro = self.outro.clone();
    new_outro.append_str(str);
    self.outro = new_outro;
  }
}

impl ToString for MagicString {
  fn to_string(&self) -> String {
    let mut str = self.intro.to_string();
    let guard = self.first_chunk.lock();
    let mut chunk = Some(&*guard);

    while let Some(c) = chunk {
      str += &c.to_string();
      chunk = c.next();
    }

    str += &self.outro.to_string();
    str
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn to_string() {
    let mut magic_string = MagicString::new(
      "hello world",
      Some(MagicStringOptions {
        filename: Some("./index.js".to_string()),
        ..Default::default()
      }),
    );
    magic_string.append("!");
    magic_string.prepend("/* ");
    magic_string.append(" */");

    let magic_map = magic_string
      .generate_map(SourceMapOptions {
        include_content: Some(true),
        file: Some("a.ts".to_string()),
        ..Default::default()
      })
      .unwrap();
    println!("==={:?}", magic_map);
    assert_eq!(magic_string.to_string(), "/* hello world! */");
  }
}
