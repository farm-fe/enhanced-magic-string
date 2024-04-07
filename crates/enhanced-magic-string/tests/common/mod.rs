use std::path::PathBuf;

use relative_path::RelativePath;

/// @deprecated using macro fixture instead
pub fn fixture<F>(pattern: &str, mut op: F)
where
  F: FnMut(PathBuf, PathBuf),
{
  let base_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
  let abs_pattern = RelativePath::new(pattern).to_path(base_dir.clone());
  let paths = glob::glob(&abs_pattern.to_string_lossy()).unwrap();

  for path in paths {
    op(path.unwrap(), base_dir.clone());
  }
}

#[macro_export]
macro_rules! fixture {
  ($pattern:expr, $op:expr) => {
    if cfg!(debug_assertions) {
      crate::common::fixture_debug($pattern, file!(), $op);
      return;
    }

    crate::common::fixture($pattern, $op);
  };
}

/// @deprecated using macro fixture instead
pub fn fixture_debug<F>(pattern: &str, test_file_path: &str, mut op: F)
where
  F: FnMut(PathBuf, PathBuf),
{
  // find closest Cargo.toml
  let mut file_path =
    RelativePath::new(test_file_path).to_logical_path(std::env::current_dir().unwrap());
  while let Some(parent) = file_path.parent() {
    if parent.join("Cargo.toml").exists() {
      break;
    }

    file_path = parent.to_path_buf();
  }

  if file_path.parent().is_none() {
    panic!("can't find Cargo.toml");
  }

  let base_dir = file_path.parent().unwrap().to_path_buf();
  let abs_pattern = RelativePath::new(pattern).to_path(base_dir.clone());
  let paths = glob::glob(&abs_pattern.to_string_lossy()).unwrap();

  let mut exists = false;

  for path in paths {
    exists = true;
    op(path.unwrap(), base_dir.clone());
  }

  if !exists {
    panic!("no fixtures found under {}", pattern);
  }
}

// Solve some line break and path mismatch issues that occur across platforms.
pub fn normalize_newlines(input: &str) -> String {
  input.replace("\r\n", "\n").replace("\\\\", "/")
}
