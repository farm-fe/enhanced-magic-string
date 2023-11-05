use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, Eq)]
pub struct CharString {
  chars: Vec<char>,
}

impl CharString {
  pub fn new(str: &str) -> Self {
    Self {
      chars: str.chars().collect(),
    }
  }

  pub fn get(&self, index: usize) -> Option<&char> {
    self.chars.get(index)
  }

  pub fn get_mut(&mut self, index: usize) -> Option<&mut char> {
    self.chars.get_mut(index)
  }

  pub fn slice(&self, start: usize, end: usize) -> Self {
    Self {
      chars: self.chars[start..end].to_vec(),
    }
  }

  pub fn split(&self, separator: char) -> Vec<Self> {
    let mut result = vec![];
    let mut start = 0;

    for (index, char) in self.chars.iter().enumerate() {
      if *char == separator {
        result.push(self.slice(start, index));
        start = index + 1;
      }
    }

    result.push(self.slice(start, self.len()));

    result
  }

  pub fn len(&self) -> usize {
    self.chars.len()
  }

  pub fn is_empty(&self) -> bool {
    self.chars.is_empty()
  }

  pub fn insert(&mut self, index: usize, char: char) {
    self.chars.insert(index, char);
  }

  pub fn remove(&mut self, index: usize) -> Option<char> {
    if index >= self.chars.len() {
      return None;
    }

    Some(self.chars.remove(index))
  }

  pub fn append_str(&mut self, other: &str) {
    self.chars.extend(other.chars());
  }

  pub fn append(&mut self, other: &CharString) {
    self.chars.extend(other.chars.iter());
  }
}

impl From<&str> for CharString {
  fn from(str: &str) -> Self {
    Self::new(str)
  }
}

impl From<String> for CharString {
  fn from(str: String) -> Self {
    Self::new(&str)
  }
}

impl From<&String> for CharString {
  fn from(str: &String) -> Self {
    Self::new(str)
  }
}

impl From<char> for CharString {
  fn from(char: char) -> Self {
    Self { chars: vec![char] }
  }
}

impl Display for CharString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.chars.iter().collect::<String>())
  }
}

impl Debug for CharString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.chars.iter().collect::<String>())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_char_string() {
    let str = CharString::new("hello world");
    assert_eq!(str.len(), 11);
    assert_eq!(str.get(0), Some(&'h'));
    assert_eq!(str.get(1), Some(&'e'));
    assert_eq!(str.get(2), Some(&'l'));
    assert_eq!(str.get(3), Some(&'l'));
    assert_eq!(str.get(4), Some(&'o'));
    assert_eq!(str.get(5), Some(&' '));
    assert_eq!(str.get(6), Some(&'w'));
    assert_eq!(str.get(7), Some(&'o'));
    assert_eq!(str.get(8), Some(&'r'));
    assert_eq!(str.get(9), Some(&'l'));
    assert_eq!(str.get(10), Some(&'d'));
    assert_eq!(str.get(11), None);

    let mut str = CharString::new("hello world");
    str.insert(5, '!');
    assert_eq!(str.to_string(), "hello! world");

    let mut str = CharString::new("hello world");
    str.remove(5);
    assert_eq!(str.to_string(), "helloworld");

    let str = CharString::new("hello world");
    assert_eq!(str.slice(0, 5).to_string(), "hello");
    assert_eq!(str.slice(6, 11).to_string(), "world");
    assert_eq!(str.slice(0, 11).to_string(), "hello world");
  }

  #[test]
  fn test_split() {
    let str = CharString::new("hello world");
    let result = str.split(' ');
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].to_string(), "hello");
    assert_eq!(result[1].to_string(), "world");

    let str = CharString::new("hello world");
    let result = str.split('o');
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].to_string(), "hell");
    assert_eq!(result[1].to_string(), " w");
    assert_eq!(result[2].to_string(), "rld");

    let str = CharString::new("\n");
    let result = str.split('\n');
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].to_string(), "");
    assert_eq!(result[1].to_string(), "");
  }
}
