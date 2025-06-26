use wasm_bindgen::prelude::*;

pub mod docx;
pub mod error;
pub mod template;
pub mod utils;

pub use docx::DocxProcessor;
pub use error::{DocxHandlebarsError, Result};
pub use template::TemplateEngine;



/// 当 `console_error_panic_hook` 功能启用时，我们可以调用 `set_panic_hook` 函数
/// 至少一次在初始化过程中，以便在 panic 时获得更好的错误消息。
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// 主要的 DOCX Handlebars 处理器
#[wasm_bindgen]
pub struct DocxHandlebars {
    processor: DocxProcessor,
    template_engine: TemplateEngine,
}

#[wasm_bindgen]
impl DocxHandlebars {
    /// 创建新的 DocxHandlebars 实例
    #[wasm_bindgen(constructor)]
    pub fn new() -> DocxHandlebars {
        set_panic_hook();
        
        DocxHandlebars {
            processor: DocxProcessor::new(),
            template_engine: TemplateEngine::new(),
        }
    }

    /// 加载 DOCX 模板文件
    #[wasm_bindgen]
    pub fn load_template(&mut self, bytes: &[u8]) -> std::result::Result<(), JsValue> {
        self.processor.load_from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// 使用给定数据渲染模板
    #[wasm_bindgen]
    pub fn render(&self, data_json: &str) -> std::result::Result<Vec<u8>, JsValue> {
        let data: serde_json::Value = serde_json::from_str(data_json)
            .map_err(|e| JsValue::from_str(&format!("JSON 解析错误: {}", e)))?;
        
        let rendered_content = self.template_engine.render_content(
            self.processor.get_content(),
            &data
        ).map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        self.processor.create_docx_with_content(&rendered_content)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// 获取模板中的变量列表
    #[wasm_bindgen]
    pub fn get_template_variables(&self) -> std::result::Result<String, JsValue> {
        let variables = self.template_engine.extract_variables(self.processor.get_content())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        serde_json::to_string(&variables)
            .map_err(|e| JsValue::from_str(&format!("序列化错误: {}", e)))
    }
}

// Rust 原生 API
impl DocxHandlebars {
    /// 从字节数组加载模板
    pub fn load_template_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.processor.load_from_bytes(bytes)?;
        Ok(())
    }

    /// 使用结构化数据渲染模板
    pub fn render_with_data<T: serde::Serialize>(&self, data: &T) -> Result<Vec<u8>> {
        let json_value = serde_json::to_value(data)?;
        let rendered_content = self.template_engine.render_content(
            self.processor.get_content(),
            &json_value
        )?;
        
        self.processor.create_docx_with_content(&rendered_content)
    }

    /// 获取模板变量
    pub fn extract_template_variables(&self) -> Result<Vec<String>> {
        self.template_engine.extract_variables(self.processor.get_content())
    }
}

impl Default for DocxHandlebars {
    fn default() -> Self {
        Self::new()
    }
}
