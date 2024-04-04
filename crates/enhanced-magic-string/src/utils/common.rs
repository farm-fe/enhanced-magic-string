use std::path::{Component, Path, PathBuf};

pub fn get_relative_path(from: &str, to: &str) -> Option<String> {
  let from_path = Path::new(from);
  let to_path = Path::new(to);

  let mut from_iter: Vec<Component> = from_path.components().collect();
  let mut to_iter: Vec<Component> = to_path.components().collect();

  from_iter.pop();

  while from_iter.len() != 0 && from_iter.first() == to_iter.first() {
    from_iter.remove(0);
    to_iter.remove(0);
  }

  let mut result = PathBuf::new();

  for _ in from_iter {
    result.push("..");
  }

  for path in to_iter {
    result.push(path.as_os_str());
  }

  Some(result.to_string_lossy().to_string())
}
