import { 模块1 } from "./modules/m-1";
import { 模块2 } from "./modules/m-2";

function 主要入口() {
  console.log(模块1());
  console.log(模块2());
}

主要入口();
export function 模块1() {
  console.log('in 模块1');
  return "模块1";
}
export function 模块2() {
  debugger;
  console.log('在模块2中');
  console.log('也在模块2中');
  return "模块2";
}