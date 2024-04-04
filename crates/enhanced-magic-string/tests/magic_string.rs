use std::fs;

use enhanced_magic_string::{
  magic_string::{MagicString, MagicStringOptions},
  types::SourceMapOptions,
};

mod common;

#[test]
fn magic_string() {
  fixture!("tests/fixtures/magic-string/basic.js", |file, _| {
    let content = fs::read_to_string(&file).unwrap();
    let dir = file.parent().unwrap();
    let filename = Some(String::from("./fixtures/magic-string/basic.js"));

    let mut s = MagicString::new(
      &content,
      Some(MagicStringOptions {
        filename: filename.clone(),
        ..Default::default()
      }),
    );

    s.prepend("/* Are you ok? */\n");
    s.append("/* this is magic string */\n");

    let map = s
      .generate_map(SourceMapOptions {
        include_content: Some(true),
        file: Some(String::from("basic.js.map")),
        source: filename.clone(),
        ..Default::default()
      })
      .unwrap();

    let code = s.to_string();

    let expect_code = fs::read_to_string(dir.join("basic.output.js")).unwrap();

    let mut str_buf = vec![];
    map.to_writer(&mut str_buf).unwrap();
    let map_str = String::from_utf8(str_buf).unwrap();
    let expect_map = fs::read_to_string(dir.join("basic.js.map")).unwrap();

    assert_eq!(code, expect_code);
    assert_eq!(map_str, expect_map.replace(";\"}", "\"}"));
  });
}
