import {relative} from 'path';
import test from 'ava';

test.skip('this is a utils file',t=>t.is(true, true));

export const get_relative_path = (dir, path) => {
  return relative(dir, path).replace(/\\/g, '/');
}
