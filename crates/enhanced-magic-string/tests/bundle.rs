use std::sync::Arc;

use enhanced_magic_string::{
  bundle::BundleOptions,
  magic_string::{MagicString, MagicStringOptions},
  types::{MappingsOptionHires, SourceMapOptions},
};
use farmfe_utils::relative;
use parking_lot::Mutex;

mod common;

#[test]
fn bundle() {
  fixture!("tests/fixtures/bundle/**/input.js", |file, _| {
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
        let mut m = MagicString::new(
          &content,
          Some(MagicStringOptions {
            filename: Some(relative(
              dir.to_string_lossy().to_string().as_str(),
              path.to_str().unwrap(),
            )),
            ..Default::default()
          }),
        );
        m.prepend("/* module */");
        m.append("/* end of module */");
        modules.push(m);
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
    bundle.add_source(magic_string, None).unwrap();

    modules.into_iter().for_each(|module| {
      bundle.add_source(module, None).unwrap();
    });

    let mut header = "/* header */\n".to_owned();

    if cfg!(target_os = "windows") {
      header = header.replace("\n", "\r\n");
    }

    bundle.prepend(&header);
    bundle.append("//# sourceMappingURL=output.js.map", None);

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
    assert_eq!(map_str, expected_map.replace(";\"}", "\"}"));
  });
}

#[test]
fn bundle_multi_thread() {
  let bundle = Mutex::new(enhanced_magic_string::bundle::Bundle::new(
    BundleOptions::default(),
  ));

  std::thread::scope(|s| {
    s.spawn(|| {
      let mut a = MagicString::new("a", None);
      a.prepend("/* ");
      a.append(" */");

      bundle.lock().add_source(a, None).unwrap();
    });
  });

  std::thread::scope(|s| {
    s.spawn(|| {
      let mut b = MagicString::new("b", None);
      b.prepend("/* ");
      b.append(" */");

      bundle.lock().add_source(b, None).unwrap();
    });
  });

  let code = bundle.lock().to_string();
  assert_eq!(code, "/* a */\n/* b */");
}

#[test]
fn combine_string_with_original_sourcemap() {
  fixture!("tests/fixtures/combine-string/**/input.js", |file, _| {
    println!("[combine string test] file: {:?}", file);
    // read files under modules directory
    let mut modules = vec![];

    let dir = file.parent().unwrap();
    let modules_dir = dir.join("modules");

    if modules_dir.exists() {
      let paths = glob::glob(&modules_dir.join("**/*.js").to_string_lossy()).unwrap();

      for path in paths {
        let path = path.unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let sourcemap_content = std::fs::read_to_string(&path.with_extension("js.map")).unwrap();
        let mut m = MagicString::new(
          &content,
          Some(MagicStringOptions {
            filename: Some(relative(
              dir.to_string_lossy().to_string().as_str(),
              path.to_str().unwrap(),
            )),
            source_map_chain: vec![Arc::new(sourcemap_content)],
            ..Default::default()
          }),
        );
        m.prepend("/* module */");
        m.append("/* end of module */");
        modules.push(m);
      }
    }

    let file_content = std::fs::read_to_string(&file).unwrap();
    let sourcemap_content = std::fs::read_to_string(&file.with_extension("js.map")).unwrap();
    let magic_string = MagicString::new(
      &file_content,
      Some(MagicStringOptions {
        filename: Some(relative(
          dir.to_string_lossy().to_string().as_str(),
          file.to_str().unwrap(),
        )),
        source_map_chain: vec![Arc::new(sourcemap_content)],
        ..Default::default()
      }),
    );
    let mut bundle = enhanced_magic_string::bundle::Bundle::new(BundleOptions {
      trace_source_map_chain: Some(true),
      ..Default::default()
    });
    bundle.add_source(magic_string, None).unwrap();

    modules.into_iter().for_each(|module| {
      bundle.add_source(module, None).unwrap();
    });

    bundle.prepend("/* header */\n");
    bundle.append("//# sourceMappingURL=output.js.map", None);

    let code = bundle.to_string();
    let map = bundle
      .generate_map(SourceMapOptions {
        include_content: Some(true),
        hires: Some(MappingsOptionHires::Boundary),
        ..Default::default()
      })
      .unwrap();
    let mut src_buf = vec![];
    map.to_writer(&mut src_buf).unwrap();
    let map_str = String::from_utf8(src_buf).unwrap();

    if !dir.join("output.js").exists() {
      std::fs::write(dir.join("output.js"), &code).unwrap();
    }

    let expected = std::fs::read_to_string(dir.join("output.js")).unwrap();
    assert_eq!(code, expected);

    if !dir.join("output.js.map").exists() {
      std::fs::write(dir.join("output.js.map"), &map_str).unwrap();
    }

    let expected_map = std::fs::read_to_string(dir.join("output.js.map")).unwrap();
    assert_eq!(map_str, expected_map.replace(";\"}", "\"}"));
  });
}
