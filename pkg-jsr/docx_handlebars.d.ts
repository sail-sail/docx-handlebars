/* tslint:disable */
/* eslint-disable */
/**
 * 主要的 DOCX Handlebars 处理器
 */
export class DocxHandlebars {
  free(): void;
  /**
   * 创建新的 DocxHandlebars 实例
   */
  constructor();
  /**
   * 加载 DOCX 模板文件
   */
  load_template(bytes: Uint8Array): void;
  /**
   * 使用给定数据渲染模板
   */
  render(data_json: string): Uint8Array;
  /**
   * 获取模板中的变量列表
   */
  get_template_variables(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_docxhandlebars_free: (a: number, b: number) => void;
  readonly docxhandlebars_new: () => number;
  readonly docxhandlebars_load_template: (a: number, b: number, c: number) => [number, number];
  readonly docxhandlebars_render: (a: number, b: number, c: number) => [number, number, number, number];
  readonly docxhandlebars_get_template_variables: (a: number) => [number, number, number, number];
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
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
