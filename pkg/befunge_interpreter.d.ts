/* tslint:disable */
/* eslint-disable */
/**
* @returns {boolean} 
*/
export function isStepMode(): boolean;
/**
* @param {string} input 
* @param {string} pre_code 
*/
export function read(input: string, pre_code: string): void;
/**
* @param {BigInt64Array} _stack 
*/
export function stack(_stack: BigInt64Array): void;
/**
* @param {string} output 
*/
export function output(output: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly isStepMode: () => number;
  readonly read: (a: number, b: number, c: number, d: number) => void;
  readonly stack: (a: number, b: number) => void;
  readonly output: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0d45cf00446aaadd: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        