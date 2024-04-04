import path,{relative} from 'path';
import fs, { writeFileSync } from 'fs';
import test from 'ava'
import fg from 'fast-glob';
import MagicString, { Bundle } from 'magic-string';
import { fileURLToPath } from 'url';
import {get_relative_path} from './common/utils.mjs';

export const currentDir = path.dirname(fileURLToPath(import.meta.url));

test('bundle of expected magic-string result', (t) => {
  const paths = fg.sync("fixtures/bundle/**/input.js", { cwd: currentDir, absolute: true });

  paths.forEach((inputPath) => {
    const dir = path.dirname(inputPath);

    const modulesDir = path.join(path.dirname(inputPath), 'modules');
    const modules = fs.readdirSync(modulesDir).map((module) => {
      return path.join(modulesDir, module)
    })

    const moduleContents = modules.filter(m => m.endsWith(".js")).map((module) => {
      return {
        path: get_relative_path(dir, module),
        content: fs.readFileSync(module, 'utf-8')
      }
    });

    const inputContent = fs.readFileSync(inputPath, 'utf-8');
    const input = new MagicString(inputContent, { filename: get_relative_path(dir, inputPath) });
    const bundle = new Bundle();
    bundle.addSource(input);

    moduleContents.forEach((module) => {
      const m = new MagicString(module.content, { filename: module.path });
      m.prepend("/* module */");
      m.append("/* end of module */");
      bundle.addSource(m);
    });


    bundle.prepend("/* header */\n");
    bundle.append("//# sourceMappingURL=output.js.map");

    const output = bundle.toString();
    const map = bundle.generateMap({ includeContent: true });

    writeFileSync(inputPath.replace('input.js', 'output.js'), output);
    writeFileSync(inputPath.replace('input.js', 'output.js.map'), map.toString());

    t.is(true, true);
  });
})

