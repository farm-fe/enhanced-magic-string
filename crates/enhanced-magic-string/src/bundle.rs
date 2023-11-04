use std::collections::{HashMap, HashSet};

use farmfe_utils::relative;
use sourcemap::{SourceMap, SourceMapBuilder};

use crate::{
  error::{Error, Result},
  magic_string::MagicString,
  mappings::{Mappings, MappingsOptionHires, SourceMapOptions},
  utils::{char_string::CharString, get_locator::get_locator},
};

#[derive(Default)]
pub struct BundleOptions {
  pub separator: Option<char>,
  pub intro: Option<CharString>,
}

struct UniqueSource {
  pub filename: String,
  pub content: CharString,
}

pub struct Bundle {
  separator: char,
  intro: CharString,
  sources: Vec<MagicString>,
  unique_sources: Vec<UniqueSource>,
  unique_source_index_by_filename: HashMap<String, usize>,
}

impl Bundle {
  pub fn new(options: BundleOptions) -> Self {
    Self {
      separator: options.separator.unwrap_or('\n'),
      intro: options.intro.unwrap_or("".into()),
      sources: vec![],
      unique_sources: vec![],
      unique_source_index_by_filename: HashMap::new(),
    }
  }

  pub fn add_source(&mut self, source: MagicString) -> Result<()> {
    if let Some(filename) = &source.filename {
      if let Some(index) = self.unique_source_index_by_filename.get(filename) {
        let unique_source = &self.unique_sources[*index];

        if unique_source.content != source.original {
          return Err(Error::IllegalSource);
        }
      } else {
        self
          .unique_source_index_by_filename
          .insert(filename.clone(), self.unique_sources.len());
        self.unique_sources.push(UniqueSource {
          filename: filename.clone(),
          content: source.original.clone(),
        });
      }
    }

    self.sources.push(source);

    Ok(())
  }

  pub fn generate_map(&self, opts: SourceMapOptions) -> Result<SourceMap> {
    let mut names = vec![];
    // let mut x_google_ignoreList = None;

    self.sources.iter().for_each(|source| {
      source.stored_names.iter().for_each(|(name, _)| {
        names.push(name.clone());
      });
    });

    let mut mappings = Mappings::new(opts.hires.unwrap_or_default());

    if !self.intro.is_empty() {
      mappings.advance(&self.intro);
    }

    self.sources.iter().enumerate().for_each(|(i, source)| {
      if i > 0 {
        mappings.advance(&self.separator.into());
      }

      let source_index: isize = if let Some(filename) = &source.filename {
        (*self.unique_source_index_by_filename.get(filename).unwrap())
          .try_into()
          .unwrap()
      } else {
        -1
      };
      let locate = get_locator(&source.original);

      if !source.intro.is_empty() {
        mappings.advance(&source.intro);
      }

      source.first_chunk.each_next(|chunk| {
        let loc = locate(chunk.start);

        if !chunk.intro.is_empty() {
          mappings.advance(&chunk.intro);
        }

        if source.filename.is_some() {
          if chunk.edited {
            unimplemented!("chunk.edited");
          } else {
            mappings.add_unedited_chunk(
              source_index,
              chunk,
              &source.original,
              loc,
              &source.sourcemap_locations,
            );
          }
        } else {
          mappings.advance(&chunk.content);
        }

        if !chunk.outro.is_empty() {
          mappings.advance(&chunk.outro);
        }
      });

      if !source.outro.is_empty() {
        mappings.advance(&source.outro);
      }

      // if !source.ignore_list.is_empty() {
      //   if x_google_ignoreList.is_none() {
      //     x_google_ignoreList = Some(vec![]);
      //   }

      //   x_google_ignoreList.as_mut().unwrap().push(source_index);
      // }
    });

    let mut sourcemap_builder = SourceMapBuilder::new(opts.file.as_ref().map(|f| f.as_str()));

    self.unique_sources.iter().for_each(|source| {
      let filename = if let Some(file) = &opts.file {
        relative(file, &source.filename)
      } else {
        source.filename.clone()
      };
      let src_id = sourcemap_builder.add_source(&filename);
      let inline_content = opts.include_content.unwrap_or(false);
      let content = if inline_content {
        Some(source.content.to_string())
      } else {
        None
      };
      sourcemap_builder.set_source_contents(src_id, content.as_deref());
    });

    names.into_iter().for_each(|name| {
      sourcemap_builder.add_name(&name.to_string());
    });

    mappings.into_sourcemap_mappings(&mut sourcemap_builder);

    Ok(sourcemap_builder.into_sourcemap())
  }
}

impl ToString for Bundle {
  fn to_string(&self) -> String {
    let body = self
      .sources
      .iter()
      .enumerate()
      .map(|(i, source)| {
        let separator = if i > 0 {
          self.separator.to_string()
        } else {
          "".to_string()
        };

        format!("{}{}", separator, source.to_string())
      })
      .collect::<Vec<_>>()
      .join("");

    format!("{}{}", self.intro, body)
  }
}
