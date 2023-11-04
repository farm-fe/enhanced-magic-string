import { fileURLToPath } from 'url';
import path, { relative } from 'path';
import fs, { writeFileSync } from 'fs';

import test from 'ava'
import fg from 'fast-glob';
import MagicString, { Bundle } from 'magic-string';

const currentDir = path.dirname(fileURLToPath(import.meta.url));

test('expected magic-string result', (t) => {
  const paths = fg.sync("fixtures/**/input.js", { cwd: currentDir, absolute: true });

  paths.forEach((inputPath) => {
    const dir = path.dirname(inputPath);
    const get_relative_path = (path) => {
      return relative(dir, path).replace(/\\/g, '/');
    }
    const modulesDir = path.join(path.dirname(inputPath), 'modules');
    const modules = fs.readdirSync(modulesDir).map((module) => path.join(modulesDir, module));

    const moduleContents = modules.map((module) => {
      return {
        path: get_relative_path(module),
        content: fs.readFileSync(module, 'utf-8')
      }
    });

    const inputContent = fs.readFileSync(inputPath, 'utf-8');
    const input = new MagicString(inputContent, { filename: get_relative_path(inputPath) });
    const bundle = new Bundle();
    bundle.addSource(input);

    moduleContents.forEach((module) => {
      bundle.addSource(new MagicString(module.content, { filename: module.path }));
    });

    const output = bundle.toString();
    const map = bundle.generateMap({ includeContent: true });
    
    writeFileSync(inputPath.replace('input.js', 'output.js'), output);
    writeFileSync(inputPath.replace('input.js', 'output.js.map'), map.toString());

    t.is(true, true);
  });
})
