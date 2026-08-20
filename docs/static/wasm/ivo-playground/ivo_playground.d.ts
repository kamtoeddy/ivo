/* tslint:disable */
/* eslint-disable */

export function constantsCreate(input_json: string): Promise<string>;

export function dependentsCreate(input_json: string): Promise<string>;

export function laxDefaultsCreate(input_json: string): Promise<string>;

export function requiredCreate(input_json: string): Promise<string>;

export function timestampsCreate(input_json: string): Promise<string>;

export function virtualsCreate(input_json: string): Promise<string>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly timestampsCreate: (a: number, b: number) => any;
    readonly dependentsCreate: (a: number, b: number) => any;
    readonly constantsCreate: (a: number, b: number) => any;
    readonly laxDefaultsCreate: (a: number, b: number) => any;
    readonly requiredCreate: (a: number, b: number) => any;
    readonly virtualsCreate: (a: number, b: number) => any;
    readonly wasm_bindgen_d50cc9a54e05ed87___convert__closures_____invoke___wasm_bindgen_d50cc9a54e05ed87___JsValue__core_9b3796e30d99ddb7___result__Result_____wasm_bindgen_d50cc9a54e05ed87___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_d50cc9a54e05ed87___convert__closures_____invoke___js_sys_887c7349514fc4a3___Function_fn_wasm_bindgen_d50cc9a54e05ed87___JsValue_____wasm_bindgen_d50cc9a54e05ed87___sys__Undefined___js_sys_887c7349514fc4a3___Function_fn_wasm_bindgen_d50cc9a54e05ed87___JsValue_____wasm_bindgen_d50cc9a54e05ed87___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
