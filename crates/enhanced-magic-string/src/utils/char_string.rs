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

  pub fn len(&self) -> usize {
    self.chars.len()
  }

  pub fn is_empty(&self) -> bool {
    self.chars.is_empty()
  }

  pub fn push(&mut self, char: char) {
    self.chars.push(char);
  }

  pub fn pop(&mut self) -> Option<char> {
    self.chars.pop()
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

  pub fn to_string(&self) -> String {
    self.chars.iter().collect()
  }
}
