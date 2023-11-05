/* header */
import { m1 } from "./modules/m-1";
import { m2 } from "./modules/m-2";

function main() {
  console.log(m1());
  console.log(m2());
}

main();
export function m1() {
  console.log('in m1');
  return "m1";
}
export function m2() {
  debugger;
  console.log('in m2');
  console.log('in m2 too');
  return "m2";
}//# sourceMappingURL=output.js.map