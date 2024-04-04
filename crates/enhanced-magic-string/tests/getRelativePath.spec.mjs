import test from "ava";
import { writeFileSync } from "fs";
import path from "path";
import { fileURLToPath } from 'url';

export const currentDir = path.dirname(fileURLToPath(import.meta.url));

export function getRelativePath(from, to) {
	const fromParts = from.split(/[/\\]/);
	const toParts = to.split(/[/\\]/);

	fromParts.pop(); // get dirname

	while (fromParts.length && fromParts[0] === toParts[0]) {
		fromParts.shift();
		toParts.shift();
	}

	if (fromParts.length) {
		let i = fromParts.length;
		while (i--) fromParts[i] = "..";
	}

	return fromParts.concat(toParts).join("/");
}

test("getRelativePath test case", (t) => {
	let code = "";
	const diff_path_list = [
		["fixtures/bundle/01/input.js", "fixtures/bundle/01/modules/a.js"],
		["output.js.map", "output.js"],
		["./common/mod.file.js", "./common/test/mod.source.js"],
		["./common/test/mod.file.js", "./common/mod.source.js"],
		["a/b/c", "a/b"],
		["/Users/xxx/enhanced-magic-string/crates/enhanced-magic-string/tests/fixtures/magic-string", "/Users/xxx/enhanced-magic-string/crates/enhanced-magic-string/tests/fixtures/magic-string/basic.js", ""]
	];

	for (const path of diff_path_list) {
		const r = getRelativePath(path[0], path[1]);
		code += `${r};`;
	}

	writeFileSync(
		path.resolve(currentDir, "./fixtures/get-relative-path/output.txt"),
		code,
	);

	t.is(true, true);
});

