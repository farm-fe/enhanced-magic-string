use std::fs;

use enhanced_magic_string::utils::common::get_relative_path;

mod common;

#[test]
fn get_relative_path_case() {
  fixture!("tests/fixtures/get-relative-path/output.txt", |file, _| {
    let expect_result = fs::read_to_string(file).unwrap();

    let from_to_paths = vec![
      (
        "fixtures/bundle/01/input.js",
        "fixtures/bundle/01/modules/a.js",
      ),
      ("output.js.map", "output.js"),
      ("./common/mod.file.js", "./common/test/mod.source.js"),
      ("./common/test/mod.file.js", "./common/mod.source.js"),
      ("a/b/c", "a/b"),
      ("/Users/xxx/enhanced-magic-string/crates/enhanced-magic-string/tests/fixtures/magic-string", "/Users/xxx/enhanced-magic-string/crates/enhanced-magic-string/tests/fixtures/magic-string/basic.js")
    ];

    let mut result = String::from("");
    for (from, to) in from_to_paths.iter() {
      let p = get_relative_path(from, to).unwrap();
      result.push_str(&p);
      result.push(';');
    }

    if cfg!(windows) {
      result = result.replace("\\", "/");
    }

    assert_eq!(result, expect_result);
  });
}
