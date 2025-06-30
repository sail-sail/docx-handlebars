use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub mod template;
pub mod utils;

/// 当 `console_error_panic_hook` 功能启用时，我们可以调用 `set_panic_hook` 函数
/// 至少一次在初始化过程中，以便在 panic 时获得更好的错误消息。
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// 主要的 DOCX Handlebars 处理器
#[wasm_bindgen]
pub fn render(
    zip_bytes: Vec<u8>,
    data_json: &str,
) -> Result<JsValue, JsValue> {
    let data: serde_json::Value = serde_json::from_str(data_json)
            .map_err(|e| JsValue::from_str(&format!("JSON 解析错误: {}", e)))?;

    // 调用模板渲染函数
    let result = template::render_handlebars(zip_bytes, &data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 返回结果
    Ok(JsValue::from(result))
}
