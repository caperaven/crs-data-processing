/* tslint:disable */
/* eslint-disable */
/**
* @param {any} intent
* @param {any} row
* @param {boolean} case_sensitive
* @returns {boolean}
*/
export function in_filter(intent: any, row: any, case_sensitive: boolean): boolean;
/**
* @param {Array<any>} data
* @param {any} intent
* @param {boolean} case_sensitive
* @returns {Array<any>}
*/
export function filter(data: Array<any>, intent: any, case_sensitive: boolean): Array<any>;
/**
* @param {Array<any>} data
* @param {any} intent
* @param {Uint32Array | undefined} rows
* @returns {Uint32Array}
*/
export function sort(data: Array<any>, intent: any, rows?: Uint32Array): Uint32Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly in_filter: (a: number, b: number, c: number, d: number) => void;
  readonly filter: (a: number, b: number, c: number, d: number) => void;
  readonly sort: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
