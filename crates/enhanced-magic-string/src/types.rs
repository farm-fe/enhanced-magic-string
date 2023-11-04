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
