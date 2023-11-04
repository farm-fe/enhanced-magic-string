use std::collections::HashSet;

use regex::Regex;
use sourcemap::SourceMapBuilder;

use crate::{
  chunk::Chunk,
  utils::{char_string::CharString, get_locator::Loc},
};

/// Whether the mapping should be high-resolution.
/// Hi-res mappings map every single character, meaning (for example) your devtools will always
/// be able to pinpoint the exact location of function calls and so on.
/// With lo-res mappings, devtools may only be able to identify the correct
/// line - but they're quicker to generate and less bulky.
/// You can also set `"boundary"` to generate a semi-hi-res mappings segmented per word boundary
/// instead of per character, suitable for string semantics that are separated by words.
/// If sourcemap locations have been specified with s.addSourceMapLocation(), they will be used here.
pub enum MappingsOptionHires {
  Bool(bool),
  Boundary,
}

impl Default for MappingsOptionHires {
  fn default() -> Self {
    Self::Bool(false)
  }
}

impl MappingsOptionHires {
  pub fn is_boundary(&self) -> bool {
    match self {
      Self::Bool(_) => false,
      Self::Boundary => true,
    }
  }

  pub fn is_truthy(&self) -> bool {
    match self {
      Self::Bool(b) => *b,
      Self::Boundary => true,
    }
  }
}

pub type RawSegment = Vec<usize>;
pub type RawSegments = Vec<RawSegment>;

pub struct Mappings {
  hires: MappingsOptionHires,
  generated_code_line: usize,
  generated_code_column: usize,
  raw: Vec<RawSegments>,
  pending: Option<usize>,
  word_regex: Regex,
}

impl Mappings {
  pub fn new(hires: MappingsOptionHires) -> Self {
    Self {
      hires,
      generated_code_line: 0,
      generated_code_column: 0,
      raw: vec![vec![]],
      pending: None,
      word_regex: Regex::new(r"\w").unwrap(),
    }
  }

  pub(crate) fn push_segment(&mut self, segment: RawSegment) {
    self.raw[self.generated_code_line].push(segment);
  }

  pub(crate) fn inc_generated_code_line(&mut self) {
    self.generated_code_line += 1;
    self.generated_code_column = 0;
    self.raw.push(vec![]);
  }

  pub fn add_unedited_chunk(
    &mut self,
    source_index: isize,
    chunk: &Chunk,
    original: &CharString,
    mut loc: Loc,
    sourcemap_locations: &HashSet<usize>,
  ) {
    let mut original_char_index = chunk.start;
    let mut first = true;
    let mut char_in_hires_boundary = false;

    while original_char_index < chunk.end {
      let char = original.get(original_char_index).unwrap();

      if self.hires.is_truthy() || first || sourcemap_locations.contains(&original_char_index) {
        let segment = vec![
          self.generated_code_column,
          source_index as usize,
          loc.line,
          loc.column,
        ];

        if self.hires.is_boundary() {
          if self.word_regex.is_match(char.to_string().as_str()) {
            if !char_in_hires_boundary {
              self.push_segment(segment);
              char_in_hires_boundary = true;
            }
          } else {
            self.push_segment(segment);
            char_in_hires_boundary = false;
          }
        } else {
          self.push_segment(segment);
        }
      }

      if *char == '\n' {
        loc.line += 1;
        loc.column = 0;
        self.inc_generated_code_line();
        first = true;
      } else {
        loc.column += 1;
        self.generated_code_column += 1;
        first = false;
      }

      original_char_index += 1;
    }

    self.pending = None;
  }

  pub fn advance(&mut self, str: &CharString) {
    if str.is_empty() {
      return;
    }

    let lines = str.split('\n');

    if lines.len() > 1 {
      for _ in 1..lines.len() {
        self.inc_generated_code_line();
      }
    }

    self.generated_code_column += lines.last().unwrap().len();
  }

  pub(crate) fn into_sourcemap_mappings(self, builder: &mut SourceMapBuilder) {
    let mut generated_code_line = 0u32;

    for segments in self.raw {
      for segment in segments {
        let dst_line = generated_code_line;
        let dst_col = segment[0];
        let src_line = segment[2];
        let src_col = segment[3];
        let src_id = segment[1];
        let name_id = segment.get(4).map(|id| *id as u32);

        builder.add_raw(
          dst_line as u32,
          dst_col as u32,
          src_line as u32,
          src_col as u32,
          Some(src_id as u32),
          name_id,
        );
      }

      generated_code_line += 1;
    }
  }
}

#[derive(Default)]
pub struct SourceMapOptions {
  pub hires: Option<MappingsOptionHires>,

  /// The filename where you plan to write the sourcemap.
  pub file: Option<String>,

  /// The filename of the file containing the original source.
  pub source: Option<String>,

  /// Whether to include the original content in the map's sourcesContent array.
  pub include_content: Option<bool>,
}
