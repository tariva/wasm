import * as wasm from "./wasmdemo_bg.wasm";
import { __wbg_set_wasm } from "./wasmdemo_bg.js";
__wbg_set_wasm(wasm);
export * from "./wasmdemo_bg.js";

wasm.__wbindgen_start();
