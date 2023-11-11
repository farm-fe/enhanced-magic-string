use std::{
  cell::{RefCell, RefMut},
  path::PathBuf,
};

use farmfe_utils::file_url_to_path;
use sourcemap::{SourceMap, SourceMapBuilder, Token};

pub struct CollapseSourcemapOptions {
  /// if true, inline source content to the source map.
  /// if the source content does not exist and source filename exists, content will be read from source file from disk.
  pub inline_content: bool,

  pub remap_source: Option<Box<dyn Fn(&str) -> String>>,
}

impl Default for CollapseSourcemapOptions {
  fn default() -> Self {
    Self {
      inline_content: true,
      remap_source: None,
    }
  }
}

/// collapse source map chain to a single source.
///
/// transformation: a -> b -> c -> d, source content is a and dest content is d.
/// corresponding input source map: [map_a, map_b, map_c, map_d].
///
/// now we have d and map_d, we want to get a and map_a, we should tracing from map_d to map_a.
///
pub fn collapse_sourcemap_chain(
  mut chain: Vec<SourceMap>,
  opts: CollapseSourcemapOptions,
) -> SourceMap {
  chain.reverse();
  chain = chain
    .into_iter()
    .filter(|map| map.get_token_count() > 0)
    .collect();

  if chain.is_empty() {
    panic!("source map chain is empty");
  }

  let dest_map = &chain[0];
  let mut builder = SourceMapBuilder::new(None);
  let mut mapped_src_cache = std::collections::HashMap::new();

  // trace all tokens in cur and update
  for token in dest_map.tokens() {
    let mut last_map_token = token;
    let mut completed_trace = true;

    if chain.len() > 1 {
      for map in &chain[1..] {
        if let Some(map_token) = lookup_token(
          map,
          last_map_token.get_src_line(),
          last_map_token.get_src_col(),
        ) {
          last_map_token = map_token;
        } else {
          completed_trace = false;
          break;
        }
      }
    }

    // if we can't trace back to the first map, ignore this token
    if !completed_trace {
      // builder.add_token(&token, true);
      continue;
    }

    let source = last_map_token.get_source();
    let mut srd_id = None;

    if let Some(src) = source {
      let remapped_src = if let Some(remap_source) = &opts.remap_source {
        mapped_src_cache
          .entry(src)
          .or_insert_with(|| remap_source(src))
          .to_string()
      } else {
        src.to_string()
      };

      srd_id = Some(builder.add_source(&remapped_src));
    }

    let mut name_id = None;

    if let Some(name) = last_map_token.get_name().or(token.get_name()) {
      name_id = Some(builder.add_name(name));
    }

    let added_token = builder.add_raw(
      token.get_dst_line(),
      token.get_dst_col(),
      last_map_token.get_src_line(),
      last_map_token.get_src_col(),
      srd_id,
      name_id,
    );

    if opts.inline_content && srd_id.is_some() && !builder.has_source_contents(srd_id.unwrap()) {
      let src_content = read_source_content(last_map_token, chain.last().unwrap());

      if let Some(src_content) = src_content {
        builder.set_source_contents(added_token.src_id, Some(&src_content));
      }
    }
  }

  builder.into_sourcemap()
}

/// if map_token is not exact match, we should use the token next to it to make sure the line mapping is correct.
/// this is because lookup_token of [SourceMap] will return the last found token instead of the next if it can't find exact match, which leads to wrong line mapping(mapping to previous line).
pub fn lookup_token<'a>(map: &'a SourceMap, line: u32, col: u32) -> Option<Token<'a>> {
  let token = map.lookup_token(line, col);

  if let Some(token) = token {
    // mapped to the last token of previous line.
    if token.get_dst_line() == line - 1 && token.get_dst_col() > 0 {
      let next_token = map.lookup_token(line + 1, 0);

      if let Some(next_token) = next_token {
        if next_token.get_dst_line() == line {
          return Some(next_token);
        }
      }
    }
  }

  token
}

pub fn read_source_content(token: Token<'_>, map: &SourceMap) -> Option<String> {
  if let Some(view) = token.get_source_view() {
    Some(view.source().to_string())
  } else if let Some(src) = token.get_source() {
    let src = &file_url_to_path(src);
    // try read source content from disk
    let map_file = map.get_file();

    if PathBuf::from(src).is_absolute() || map_file.is_none() {
      std::fs::read_to_string(src).ok()
    } else if let Some(map_file) = map_file {
      let src_file = PathBuf::from(map_file).parent().unwrap().join(src);
      let src_content = std::fs::read_to_string(src_file).ok();

      src_content
    } else {
      None
    }
  } else {
    None
  }
}

pub struct CollapsedSourceMap<'a> {
  pub tokens: RefCell<Vec<Token<'a>>>,
  pub map: SourceMap,
}

impl<'a> CollapsedSourceMap<'a> {
  pub fn new(map: SourceMap) -> Self {
    Self {
      tokens: RefCell::new(vec![]),
      map,
    }
  }

  pub fn tokens(&'a self) -> RefMut<Vec<Token<'a>>> {
    let mut tokens = self.tokens.borrow_mut();

    if tokens.is_empty() {
      *tokens = self.map.tokens().collect::<Vec<_>>();
    }

    tokens
  }
}
