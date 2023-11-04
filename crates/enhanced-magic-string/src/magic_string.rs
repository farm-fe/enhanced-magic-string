use std::{
  collections::{HashMap, HashSet},
  rc::Rc,
};

use crate::{
  chunk::{self, Chunk},
  utils::char_string::CharString,
};

pub type ExclusionRange = (usize, usize);

#[derive(Default)]
pub struct MagicStringOptions {
  pub filename: Option<String>,
  pub indent_exclusion_ranges: Vec<ExclusionRange>,
  pub ignore_list: Vec<CharString>,
}

pub struct MagicString {
  pub original: CharString,
  pub outro: CharString,
  pub intro: CharString,

  pub first_chunk: Rc<Chunk>,
  pub last_chunk: Rc<Chunk>,
  pub last_searched_chunk: Rc<Chunk>,
  pub chunk_by_start: HashMap<usize, Rc<Chunk>>,
  pub chunk_by_end: HashMap<usize, Rc<Chunk>>,

  pub filename: Option<String>,
  pub indent_exclusion_ranges: Vec<ExclusionRange>,
  pub sourcemap_locations: HashSet<usize>,
  pub stored_names: HashMap<CharString, bool>,
  pub indent_str: Option<CharString>,
  pub ignore_list: Vec<CharString>,
}

impl MagicString {
  pub fn new(original: &str, options: Option<MagicStringOptions>) -> Self {
    let options = options.unwrap_or_default();
    let original = CharString::new(original);
    let chunk = Rc::new(Chunk::new(0, original.len(), original.clone()));

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
    };

    magic_string
      .chunk_by_start
      .insert(0, magic_string.first_chunk.clone());
    magic_string
      .chunk_by_end
      .insert(0, magic_string.last_chunk.clone());

    magic_string
  }
}

impl ToString for MagicString {
  fn to_string(&self) -> String {
    let mut str = self.intro.to_string();
    let mut chunk = Some(self.first_chunk.as_ref());

    while let Some(c) = chunk {
      str += &c.to_string();
      chunk = c.next();
    }

    str += &self.outro.to_string();
    str
  }
}
