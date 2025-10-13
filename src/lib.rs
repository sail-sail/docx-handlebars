#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

pub mod errors;
pub mod imagesize;
mod template;
pub mod utils;

pub use errors::DocxError;

// WASM 平台：导出 WASM 兼容的渲染函数
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn render_template(
    zip_bytes: Vec<u8>,
    data_json: &str,
) -> Result<JsValue, JsValue> {
    let data: serde_json::Value = serde_json::from_str(data_json)
            .map_err(|e| JsValue::from_str(&format!("JSON Parse Error: {e}")))?;

    // 调用模板渲染函数
    let result = template::render_template(zip_bytes, &data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 返回结果
    Ok(JsValue::from(result))
}

// 非 WASM 平台：直接导出原生 Rust 函数
#[cfg(not(target_arch = "wasm32"))]
pub use template::render_template;
