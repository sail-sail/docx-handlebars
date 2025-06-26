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
