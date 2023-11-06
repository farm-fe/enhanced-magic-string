use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use parking_lot::Mutex;
use sourcemap::SourceMap;

use crate::{
  chunk::Chunk, collapse_sourcemap::collapse_sourcemap_chain, utils::char_string::CharString,
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
  collapsed_sourcemap: Mutex<Option<String>>,

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
      collapsed_sourcemap: Mutex::new(None),
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

  pub fn get_collapsed_sourcemap(&self) -> Option<SourceMap> {
    if let Some(collapsed_sourcemap) = self.collapsed_sourcemap.lock().as_ref() {
      return Some(SourceMap::from_slice(collapsed_sourcemap.as_bytes()).unwrap());
    }

    if self.source_map_chain.is_empty() {
      return None;
    }

    let source_map_chain = self
      .source_map_chain
      .iter()
      .map(|s| SourceMap::from_slice(s.as_bytes()).unwrap())
      .collect::<Vec<_>>();

    let collapsed_sourcemap = collapse_sourcemap_chain(source_map_chain, Default::default());
    let mut cached_map = self.collapsed_sourcemap.lock();

    let mut buf = vec![];
    collapsed_sourcemap.to_writer(&mut buf).unwrap();
    cached_map.replace(String::from_utf8(buf).unwrap());

    Some(collapsed_sourcemap)
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
    let mut magic_string = MagicString::new("hello world", None);
    magic_string.append("!");
    magic_string.prepend("/* ");
    magic_string.append(" */");

    assert_eq!(magic_string.to_string(), "/* hello world! */");
  }
}
