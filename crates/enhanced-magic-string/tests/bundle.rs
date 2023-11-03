use enhanced_magic_string::{
  bundle::BundleOptions,
  magic_string::{MagicString, MagicStringOptions},
  mappings::SourceMapOptions,
};

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
          content,
          Some(MagicStringOptions {
            filename: Some(
              path
                .to_string_lossy()
                .strip_prefix(dir.to_string_lossy().as_ref())
                .unwrap()
                .to_string(),
            ),
            ..Default::default()
          }),
        ));
      }
    }

    let file_content = std::fs::read_to_string(&file).unwrap();
    let magic_string = MagicString::new(
      file_content,
      Some(MagicStringOptions {
        filename: Some(
          file
            .strip_prefix(dir.to_string_lossy().as_ref())
            .unwrap()
            .to_string_lossy()
            .to_string(),
        ),
        ..Default::default()
      }),
    );
    let mut bundle = enhanced_magic_string::bundle::Bundle::new(BundleOptions::default());
    bundle.add_source(magic_string).unwrap();

    modules.into_iter().for_each(|module| {
      bundle.add_source(module).unwrap();
    });

    let code = bundle.to_string();
    println!("{}", code);
    let map = bundle.generate_map(SourceMapOptions::default()).unwrap();
    let mut src_buf = vec![];
    map.to_writer(&mut src_buf).unwrap();
    let map_str = String::from_utf8(src_buf).unwrap();
    println!("{}", map_str);
  });
}
