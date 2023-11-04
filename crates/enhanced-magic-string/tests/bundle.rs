use enhanced_magic_string::{
  bundle::BundleOptions,
  magic_string::{MagicString, MagicStringOptions},
  mappings::SourceMapOptions,
};
use farmfe_utils::relative;

mod common;

#[test]
fn bundle() {
  fixture!("tests/fixtures/bundle/**/input.js", |file, crate_path| {
    println!("[bundle test] file: {:?}", file);
    // read files under modules directory
    let mut modules = vec![];

    let dir = file.parent().unwrap();
    let modules_dir = dir.join("modules");

    if modules_dir.exists() {
      let paths = glob::glob(&modules_dir.join("**/*.js").to_string_lossy()).unwrap();

      for path in paths {
        let path = path.unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        modules.push(MagicString::new(
          &content,
          Some(MagicStringOptions {
            filename: Some(relative(
              dir.to_string_lossy().to_string().as_str(),
              path.to_str().unwrap(),
            )),
            ..Default::default()
          }),
        ));
      }
    }

    let file_content = std::fs::read_to_string(&file).unwrap();
    let magic_string = MagicString::new(
      &file_content,
      Some(MagicStringOptions {
        filename: Some(relative(
          dir.to_string_lossy().to_string().as_str(),
          file.to_str().unwrap(),
        )),
        ..Default::default()
      }),
    );
    let mut bundle = enhanced_magic_string::bundle::Bundle::new(BundleOptions::default());
    bundle.add_source(magic_string).unwrap();

    modules.into_iter().for_each(|module| {
      bundle.add_source(module).unwrap();
    });

    let code = bundle.to_string();
    let map = bundle
      .generate_map(SourceMapOptions {
        include_content: Some(true),
        ..Default::default()
      })
      .unwrap();
    let mut src_buf = vec![];
    map.to_writer(&mut src_buf).unwrap();
    let map_str = String::from_utf8(src_buf).unwrap();

    let expected = std::fs::read_to_string(dir.join("output.js")).unwrap();
    assert_eq!(code, expected);

    let expected_map = std::fs::read_to_string(dir.join("output.js.map")).unwrap();
    assert_eq!(map_str, expected_map);
  });
}
