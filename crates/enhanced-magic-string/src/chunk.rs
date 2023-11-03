use std::ptr::NonNull;

use crate::utils::char_string::CharString;

pub struct Chunk {
  pub start: usize,
  pub end: usize,
  pub original: String,

  pub intro: String,
  pub outro: String,

  pub content: String,
  pub store_name: bool,
  pub edited: bool,

  pub previous: Option<NonNull<Chunk>>,
  pub next: Option<NonNull<Chunk>>,
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
    let mut chunk = Some(self);

    while let Some(c) = chunk {
      f(&c);
      chunk = c.next();
    }
  }

  pub fn next(&self) -> Option<&Chunk> {
    match &self.next {
      Some(next) => {
        let next = unsafe { next.as_ref() };
        Some(next)
      }
      None => None,
    }
  }

  pub fn previous(&self) -> Option<&Chunk> {
    match &self.previous {
      Some(previous) => {
        let previous = unsafe { previous.as_ref() };
        Some(previous)
      }
      None => None,
    }
  }
}

impl ToString for Chunk {
  fn to_string(&self) -> String {
    format!("{}{}{}", self.intro, self.content, self.outro)
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn each_next() {
    let mut chunk = Chunk::new(0, 1, "a".to_string());
    let mut chunk2 = Chunk::new(1, 2, "b".to_string());
    let mut chunk3 = Chunk::new(2, 3, "c".to_string());

    chunk.next = Some(NonNull::new(&mut chunk2).unwrap());
    chunk2.next = Some(NonNull::new(&mut chunk3).unwrap());

    let mut result = vec![];

    chunk.each_next(|chunk| {
      result.push(chunk.content.clone());
    });

    assert_eq!(
      result,
      vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
  }
}
