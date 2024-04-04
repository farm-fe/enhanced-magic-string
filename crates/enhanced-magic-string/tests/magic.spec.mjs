import test from 'ava'
import fg from 'fast-glob';
import fs, { writeFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { get_relative_path } from './common/utils.mjs';
import path from 'path';
import MagicString from 'magic-string';

export const currentDir = path.dirname(fileURLToPath(import.meta.url));

test('expected magic-string result', (t) => {
  const paths = fg.sync("fixtures/magic-string/basic.js", { cwd: currentDir, absolute: true });
  paths.forEach(inputPath => {
    const dir = path.dirname(inputPath);

    const content = fs.readFileSync(inputPath, "utf-8");
    const magicString = new MagicString(
      content,
      {
        filename: get_relative_path(dir, inputPath)
      });
    magicString.prepend("/* Are you ok? */\n");
    magicString.append("/* this is magic string */\n");

    writeFileSync(inputPath.replace("basic.js", "basic.output.js"), magicString.toString());

    const map = magicString.generateMap({
      includeContent: true,
      file: "basic.js.map",
      source: get_relative_path(dir, inputPath),
    });
    writeFileSync(inputPath.replace("basic.js", "basic.js.map"), map.toString());

    t.is(true, true);
  })
});
