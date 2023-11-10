use std::collections::HashMap;

use farmfe_utils::relative;
use sourcemap::{SourceMap, SourceMapBuilder};

use crate::{
  collapse_sourcemap::{lookup_token, read_source_content},
  error::{Error, Result},
  magic_string::MagicString,
  mappings::Mappings,
  types::SourceMapOptions,
  utils::{char_string::CharString, get_locator::get_locator},
};

#[derive(Default)]
pub struct BundleOptions {
  pub separator: Option<char>,
  pub intro: Option<CharString>,
  pub trace_source_map_chain: Option<bool>,
}

struct UniqueSource {
  pub filename: String,
  pub content: CharString,
}

pub struct AddSourceOptions {
  pub separator: char,
  pub filename: Option<String>,
}

pub struct Bundle {
  separator: char,
  intro: CharString,
  sources: Vec<MagicString>,
  unique_sources: Vec<UniqueSource>,
  unique_source_index_by_filename: HashMap<String, usize>,
  trace_source_map_chain: bool,
}

impl Bundle {
  pub fn new(options: BundleOptions) -> Self {
    Self {
      separator: options.separator.unwrap_or('\n'),
      intro: options.intro.unwrap_or("".into()),
      sources: vec![],
      unique_sources: vec![],
      unique_source_index_by_filename: HashMap::new(),
      trace_source_map_chain: options.trace_source_map_chain.unwrap_or(false),
    }
  }

  pub fn add_source(
    &mut self,
    mut source: MagicString,
    opts: Option<AddSourceOptions>,
  ) -> Result<()> {
    let filename = opts
      .as_ref()
      .and_then(|opts| opts.filename.as_ref())
      .or(source.filename.as_ref());
    let separator = opts
      .as_ref()
      .map(|opts| opts.separator)
      .unwrap_or(self.separator);
    source.separator = separator;

    if let Some(filename) = filename {
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
        // replace \0 to empty string
        let separator = if source.separator == '\0' {
          CharString::new("")
        } else {
          CharString::from(source.separator)
        };
        mappings.advance(&separator);
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

      source.first_chunk.lock().each_next(|chunk| {
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

      if !source.ignore_list.is_empty() {
        unimplemented!("source.ignore_list");
        // if x_google_ignoreList.is_none() {
        //   x_google_ignoreList = Some(vec![]);
        // }

        // x_google_ignoreList.as_mut().unwrap().push(source_index);
      }
    });

    let mut sourcemap_builder = SourceMapBuilder::new(opts.file.as_ref().map(|f| f.as_str()));

    self.unique_sources.iter().for_each(|source| {
      let mut filename = if let Some(file) = &opts.file {
        relative(file, &source.filename)
      } else {
        source.filename.clone()
      };
      if let Some(remap_source) = &opts.remap_source {
        filename = remap_source(&filename);
      }
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

    if self.trace_source_map_chain {
      let map = sourcemap_builder.into_sourcemap();
      // try trace back to original sourcemap of each source
      let mut trace_sourcemap_builder =
        SourceMapBuilder::new(opts.file.as_ref().map(|f| f.as_str()));
      let mut collapsed_sourcemap_cache = HashMap::new();
      let mut mapped_src_cache = HashMap::new();

      for token in map.tokens() {
        if let Some(source_filename) = token.get_source() {
          if let Some(source) = self.get_source_by_filename(source_filename) {
            let source_map_chain = collapsed_sourcemap_cache
              .entry(source_filename.to_string())
              .or_insert_with(|| source.get_source_map_chain());

            if source_map_chain.is_empty() {
              trace_sourcemap_builder.add_token(&token, true);
              continue;
            }

            let mut is_trace_completed = true;
            let mut map_token = token;

            for map in source_map_chain.iter() {
              // if the token can not be found in source map chain, it will be ignored.
              if let Some(m_token) = lookup_token(map, token.get_src_line(), token.get_src_col()) {
                map_token = m_token;
              } else {
                is_trace_completed = false;
                break;
              }
            }

            if is_trace_completed {
              let src = if let Some(src) = map_token.get_source() {
                Some(if let Some(remap_source) = &opts.remap_source {
                  mapped_src_cache
                    .entry(src.to_string())
                    .or_insert_with(|| remap_source(src))
                    .to_string()
                } else {
                  src.to_string()
                })
              } else {
                None
              };

              let added_token = trace_sourcemap_builder.add(
                token.get_dst_line(),
                token.get_dst_col(),
                map_token.get_src_line(),
                map_token.get_src_col(),
                src.as_deref(),
                map_token.get_name(),
              );

              let inline_content = opts.include_content.unwrap_or(false);

              if inline_content && !trace_sourcemap_builder.has_source_contents(added_token.src_id)
              {
                let source_content =
                  read_source_content(map_token, source_map_chain.last().unwrap());

                if let Some(source_content) = source_content {
                  trace_sourcemap_builder
                    .set_source_contents(added_token.src_id, Some(&source_content));
                }
              }
            }
          }
        }
      }

      return Ok(trace_sourcemap_builder.into_sourcemap());
    }

    Ok(sourcemap_builder.into_sourcemap())
  }

  fn get_source_by_filename(&self, filename: &str) -> Option<&MagicString> {
    let source_index = self.unique_source_index_by_filename.get(filename)?;

    self.sources.get(*source_index)
  }

  pub fn append(&mut self, str: &str, opts: Option<AddSourceOptions>) {
    self
      .add_source(
        MagicString::new(str, None),
        opts.or(Some(AddSourceOptions {
          separator: '\0',
          filename: None,
        })),
      )
      .unwrap();
  }

  pub fn prepend(&mut self, str: &str) {
    let mut new_intro = CharString::new(str);
    new_intro.append(&self.intro);
    self.intro = new_intro;
  }
}

impl ToString for Bundle {
  fn to_string(&self) -> String {
    let body = self
      .sources
      .iter()
      .enumerate()
      .map(|(i, source)| {
        let separator = if i > 0 && source.separator != '\0' {
          source.separator.to_string()
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
