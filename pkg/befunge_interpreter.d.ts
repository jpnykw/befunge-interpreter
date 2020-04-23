/* tslint:disable */
/* eslint-disable */
/**
* @param {BigInt64Array} _stack 
*/
export function stack(_stack: BigInt64Array): void;
/**
* @param {string} output 
*/
export function output(output: string): void;
/**
* @param {string} input 
* @param {string} pre_code 
*/
export function read(input: string, pre_code: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly stack: (a: number, b: number) => void;
  readonly output: (a: number, b: number) => void;
  readonly read: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
        