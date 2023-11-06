use std::ptr::NonNull;

use parking_lot::Mutex;

use crate::utils::char_string::CharString;

// /// only make sure it's thread safe for T = [Chunk]
// pub struct ThreadSafeNonNull<T>(Mutex<NonNull<T>>);

// impl<T> ThreadSafeNonNull<T> {
//   pub fn new(ptr: NonNull<T>) -> Self {
//     Self(Mutex::new(ptr))
//   }

//   pub fn lock(&self) -> parking_lot::MutexGuard<NonNull<T>> {
//     self.0.lock()
//   }
// }

// unsafe impl<T> Send for ThreadSafeNonNull<T> {}
// unsafe impl<T> Sync for ThreadSafeNonNull<T> {}

pub struct Chunk {
  pub start: usize,
  pub end: usize,
  pub original: CharString,

  pub intro: CharString,
  pub outro: CharString,

  pub content: CharString,
  pub store_name: bool,
  pub edited: bool,

  previous: Mutex<Option<Box<Chunk>>>,
  next: Mutex<Option<Box<Chunk>>>,
}

impl Chunk {
  pub fn new(start: usize, end: usize, content: CharString) -> Self {
    Self {
      start,
      end,
      original: content.clone(),
      intro: CharString::new(""),
      outro: CharString::new(""),
      content,
      store_name: false,
      edited: false,
      previous: Mutex::new(None),
      next: Mutex::new(None),
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
    match self.next.lock().as_ref() {
      Some(next) => {
        let next = unsafe {
          let ptr = NonNull::new(next.as_ref() as *const Chunk as *mut Chunk).expect("null ptr");
          ptr.as_ref()
        };
        Some(next)
      }
      None => None,
    }
  }

  pub fn next_mut(&mut self) -> Option<&mut Chunk> {
    match self.next.lock().as_mut() {
      Some(next) => {
        let next = unsafe {
          let mut ptr = NonNull::new(next.as_mut() as *mut Chunk).expect("null ptr");
          ptr.as_mut()
        };
        Some(next)
      }
      None => None,
    }
  }

  pub fn previous(&self) -> Option<&Chunk> {
    match self.previous.lock().as_ref() {
      Some(previous) => {
        let previous = unsafe {
          let ptr =
            NonNull::new(previous.as_ref() as *const Chunk as *mut Chunk).expect("null ptr");
          ptr.as_ref()
        };
        Some(previous)
      }
      None => None,
    }
  }

  pub fn previous_mut(&self) -> Option<&mut Chunk> {
    match self.previous.lock().as_mut() {
      Some(previous) => {
        let previous = unsafe {
          let mut ptr = NonNull::new(previous.as_mut() as *mut Chunk).expect("null ptr");
          ptr.as_mut()
        };
        Some(previous)
      }
      None => None,
    }
  }

  pub fn set_next(&mut self, next: Box<Chunk>) {
    self.next = Mutex::new(Some(next));
  }

  pub fn set_previous(&mut self, previous: Box<Chunk>) {
    self.previous = Mutex::new(Some(previous));
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
    let mut chunk = Chunk::new(0, 1, "a".into());
    let chunk2 = Chunk::new(1, 2, "b".into());
    let chunk3 = Chunk::new(2, 3, "c".into());

    chunk.set_next(Box::new(chunk2));
    chunk.next_mut().unwrap().set_next(Box::new(chunk3));

    let mut result = vec![];

    chunk.each_next(|chunk| {
      result.push(chunk.content.to_string());
    });

    assert_eq!(
      result,
      vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
  }

  #[test]
  fn multi_thread() {
    let mut chunk = Chunk::new(0, 1, "a".into());
    let chunk2 = Chunk::new(1, 2, "b".into());
    let chunk3 = Chunk::new(2, 3, "c".into());

    chunk.set_next(Box::new(chunk2));

    std::thread::scope(|s| {
      s.spawn(|| {
        chunk.next_mut().unwrap().set_next(Box::new(chunk3));
      });
    });

    let mut result = vec![];

    chunk.each_next(|chunk| {
      result.push(chunk.content.to_string());
    });

    assert_eq!(
      result,
      vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
  }
}
