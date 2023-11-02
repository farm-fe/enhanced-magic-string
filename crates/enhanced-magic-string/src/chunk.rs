use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Chunk {
  pub start: usize,
  pub end: usize,
  pub original: String,

  pub intro: String,
  pub outro: String,

  pub content: String,
  pub store_name: bool,
  pub edited: bool,

  pub previous: Option<Box<Rc<Chunk>>>,
  pub next: Option<Box<Rc<Chunk>>>,
}

impl Chunk {
  pub fn new(start: usize, end: usize, content: String) -> Self {
    Self {
      start,
      end,
      original: content.clone(),
      intro: "".to_string(),
      outro: "".to_string(),
      content,
      store_name: false,
      edited: false,
      previous: None,
      next: None,
    }
  }

  pub fn each_next<F>(&self, mut f: F)
  where
    F: FnMut(&Chunk),
  {
    let mut chunk = self;

    while let Some(next) = chunk.next() {
      f(next);
      chunk = next;
    }
  }

  fn next(&self) -> Option<&Chunk> {
    match &self.next {
      Some(next) => Some(next.as_ref()),
      None => None,
    }
  }
}
