use std::{
  collections::{HashMap, HashSet},
  rc::Rc,
};

use crate::chunk::Chunk;

pub type ExclusionRange = (usize, usize);

#[derive(Default)]
pub struct MagicStringOptions {
  filename: Option<String>,
  indent_exclusion_ranges: Vec<ExclusionRange>,
}

pub struct MagicString {
  pub original: String,
  pub outro: String,
  pub intro: String,

  pub first_chunk: Rc<Chunk>,
  pub last_chunk: Rc<Chunk>,
  pub last_searched_chunk: Rc<Chunk>,
  pub chunk_by_start: HashMap<usize, Rc<Chunk>>,
  pub chunk_by_end: HashMap<usize, Rc<Chunk>>,

  pub filename: Option<String>,
  pub indent_exclusion_ranges: bool,
  pub sourcemap_locations: HashSet<usize>,
  pub stored_names: HashMap<String, bool>,
  pub indent_str: Option<String>,
  pub ignore_list: Vec<String>,
}

// impl MagicString {
//   pub fn new(original: String, options: Option<MagicStringOptions>) -> Self {
//     let options = options.unwrap_or_default();
//     let chunk = Rc::new(Chunk::new(0, original.len(), original.clone()));

//     let mut magic_string = Self {
//       original,
//       outro: "".to_string(),
//       intro: "".to_string(),
//       first_chunk: chunk.clone(),
//       last_chunk: chunk.clone(),
//       last_searched_chunk: chunk,
//       chunk_by_start: HashMap::new(),
//       chunk_by_end: HashMap::new(),
//       filename: options.filename,
//       indent_exclusion_ranges: options.indent_exclusion_ranges,
//       sourcemap_locations: HashSet::new(),
//       stored_names: HashMap::new(),
//       indent_str: options.indent_str,
//       ignore_list: options.ignore_list,
//     };

//     magic_string
//       .chunk_by_start
//       .insert(0, magic_string.first_chunk.clone());
//     magic_string
//       .chunk_by_end
//       .insert(0, magic_string.last_chunk.clone());

//     magic_string
//   }
// }
