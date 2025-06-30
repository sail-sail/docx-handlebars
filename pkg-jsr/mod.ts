/**
 * Deno module entry point for docx-handlebars
 * 
 * @example
 * ```typescript
 * import { render, init } from "https://deno.land/x/docx_handlebars/mod.ts";
 * 
 * // Initialize WASM module
 * await init();
 * 
 * const templateBytes = await Deno.readFile("template.docx");
 * const data = { name: "张三", company: "ABC公司" };
 * const result = render(templateBytes, JSON.stringify(data));
 * 
 * await Deno.writeFile("output.docx", new Uint8Array(result));
 * ```
 */

// Import the WASM module and its functions
import init, { render } from "./docx_handlebars.js";

// Re-export the main functions
export { render, init };
export default init;

/**
 * Deno-specific utility functions
 */
export class DocxHandlebarsUtils {
  /**
   * Load WASM module for Deno environment
   */
  static async initWasm(): Promise<void> {
    // Auto-initialize WASM in Deno
    const { default: init } = await import("./pkg-npm/docx_handlebars.js");
    await init();
  }

  /**
   * Read DOCX file from file system
   */
  static async readDocxFile(filePath: string): Promise<Uint8Array> {
    return await Deno.readFile(filePath);
  }

  /**
   * Write DOCX file to file system
   */
  static async writeDocxFile(filePath: string, data: Uint8Array): Promise<void> {
    await Deno.writeFile(filePath, data);
  }

  /**
   * Check if file exists
   */
  static async fileExists(filePath: string): Promise<boolean> {
    try {
      await Deno.stat(filePath);
      return true;
    } catch {
      return false;
    }
  }
}
