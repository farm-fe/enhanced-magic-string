use std::path::PathBuf;

use sourcemap::{SourceMap, SourceMapBuilder};

pub struct CollapseSourcemapOptions {
  /// if true, inline source content to the source map.
  /// if the source content does not exist and source filename exists, content will be read from source file from disk.
  pub inline_content: bool,
}

impl Default for CollapseSourcemapOptions {
  fn default() -> Self {
    Self {
      inline_content: true,
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

  if chain.len() == 1 {
    return chain.remove(0);
  }

  let dest_map = chain.remove(0);
  let mut builder = SourceMapBuilder::new(None);

  // trace all tokens in cur and update
  for token in dest_map.tokens() {
    let mut last_map_token = token;
    let mut completed_trace = true;

    for map in &chain {
      if let Some(map_token) =
        map.lookup_token(last_map_token.get_src_line(), last_map_token.get_src_col())
      {
        last_map_token = map_token;
      } else {
        completed_trace = false;
        break;
      }
    }

    // if we can't trace back to the first map, ignore the whole sourcemap chain.
    if !completed_trace || token == last_map_token {
      builder.add_token(&token, true);
      continue;
    }

    let source = last_map_token.get_source();
    let src_content = if opts.inline_content {
      if let Some(view) = last_map_token.get_source_view() {
        Some(view.source().to_string())
      } else if let Some(src) = last_map_token.get_source() {
        // try read source content from disk
        let map_file = chain.last().unwrap().get_file();

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
    } else {
      None
    };

    let added_token = builder.add(
      token.get_dst_line(),
      token.get_dst_col(),
      last_map_token.get_src_line(),
      last_map_token.get_src_col(),
      source,
      if last_map_token.get_name().is_some() {
        last_map_token.get_name()
      } else {
        token.get_name()
      },
    );

    if let Some(src_content) = src_content {
      builder.set_source_contents(added_token.src_id, Some(&src_content));
    }
  }

  builder.into_sourcemap()
}
