import { 模块1 } from "./modules/m-1";
import { 模块2 } from "./modules/m-2";

function 主要入口() {
  console.log(模块1());
  console.log(模块2());
}

主要入口();