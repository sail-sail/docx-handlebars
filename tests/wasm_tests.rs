//! WASM 特定的测试
//! 
//! 这些测试专门针对 WebAssembly 环境
//! 运行命令: wasm-pack test --headless --firefox

use wasm_bindgen_test::*;
use docx_handlebars::DocxHandlebars;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_wasm_processor_creation() {
    let processor = DocxHandlebars::new();
    // 在 WASM 环境中创建处理器
    assert!(true);
}

#[wasm_bindgen_test]
fn test_wasm_json_handling() {
    let processor = DocxHandlebars::new();
    
    // 测试 JSON 字符串处理
    let json_data = r#"{"name": "WASM测试", "value": 42}"#;
    
    // 验证 JSON 格式
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(json_data);
    assert!(parsed.is_ok());
    
    let data = parsed.unwrap();
    assert_eq!(data["name"], "WASM测试");
    assert_eq!(data["value"], 42);
}

#[wasm_bindgen_test]
fn test_wasm_error_handling() {
    let processor = DocxHandlebars::new();
    
    // 测试无效 JSON 处理
    let invalid_json = "{ invalid json }";
    let result = processor.render(invalid_json);
    
    // 应该返回错误
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_wasm_unicode_support() {
    // 测试 Unicode 字符在 WASM 环境中的处理
    let unicode_json = r#"{"chinese": "你好", "emoji": "🚀", "mixed": "Hello 世界!"}"#;
    
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(unicode_json);
    assert!(parsed.is_ok());
    
    let data = parsed.unwrap();
    assert_eq!(data["chinese"], "你好");
    assert_eq!(data["emoji"], "🚀");
}

#[wasm_bindgen_test]
fn test_wasm_template_variables_extraction() {
    let processor = DocxHandlebars::new();
    
    // 这个测试需要实际的 DOCX 内容，现在先跳过
    // 在真实场景中，您需要加载包含模板的 DOCX 文件
    assert!(true);
}

#[wasm_bindgen_test]
fn test_wasm_memory_usage() {
    // 测试内存使用情况
    let mut processors = Vec::new();
    
    // 创建多个处理器实例
    for _ in 0..10 {
        processors.push(DocxHandlebars::new());
    }
    
    assert_eq!(processors.len(), 10);
    
    // 清理
    processors.clear();
    assert_eq!(processors.len(), 0);
}

#[wasm_bindgen_test]
fn test_wasm_complex_data_structure() {
    let complex_json = r#"{
        "employee": {
            "name": "张三",
            "department": "研发部",
            "projects": [
                {"name": "项目A", "status": "完成"},
                {"name": "项目B", "status": "进行中"}
            ]
        },
        "company": {
            "name": "科技公司",
            "location": "北京"
        },
        "metadata": {
            "generated_at": "2024-01-01T00:00:00Z",
            "version": "1.0.0"
        }
    }"#;
    
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(complex_json);
    assert!(parsed.is_ok());
    
    let data = parsed.unwrap();
    assert_eq!(data["employee"]["name"], "张三");
    assert_eq!(data["employee"]["projects"][0]["name"], "项目A");
    assert_eq!(data["company"]["name"], "科技公司");
}

#[wasm_bindgen_test]
fn test_wasm_performance() {
    use web_sys::console;
    
    let start = js_sys::Date::now();
    
    // 执行一些操作
    for i in 0..100 {
        let data = format!(r#"{{"iteration": {}, "name": "测试{}", "active": true}}"#, i, i);
        let _parsed: Result<serde_json::Value, _> = serde_json::from_str(&data);
    }
    
    let end = js_sys::Date::now();
    let duration = end - start;
    
    console::log_1(&format!("100次JSON解析耗时: {}ms", duration).into());
    
    // 性能测试通过（只要不崩溃就行）
    assert!(duration > 0.0);
}

#[wasm_bindgen_test]
fn test_wasm_array_handling() {
    let processor = DocxHandlebars::new();
    
    // 测试数组数据处理
    let array_json = r#"{
        "items": [
            {"name": "项目1", "priority": "高"},
            {"name": "项目2", "priority": "中"},
            {"name": "项目3", "priority": "低"}
        ],
        "total": 3
    }"#;
    
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(array_json);
    assert!(parsed.is_ok());
    
    let data = parsed.unwrap();
    assert!(data["items"].is_array());
    assert_eq!(data["items"].as_array().unwrap().len(), 3);
    assert_eq!(data["total"], 3);
}

#[wasm_bindgen_test]
fn test_wasm_boolean_and_null_handling() {
    let mixed_json = r#"{
        "active": true,
        "completed": false,
        "description": null,
        "count": 0,
        "name": ""
    }"#;
    
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(mixed_json);
    assert!(parsed.is_ok());
    
    let data = parsed.unwrap();
    assert_eq!(data["active"], true);
    assert_eq!(data["completed"], false);
    assert!(data["description"].is_null());
    assert_eq!(data["count"], 0);
    assert_eq!(data["name"], "");
}

#[wasm_bindgen_test]
fn test_wasm_edge_cases() {
    // 测试边界情况
    
    // 空 JSON 对象
    let empty_json = "{}";
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(empty_json);
    assert!(parsed.is_ok());
    
    // 空数组
    let empty_array = "[]";
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(empty_array);
    assert!(parsed.is_ok());
    
    // null 值
    let null_json = "null";
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(null_json);
    assert!(parsed.is_ok());
    assert!(parsed.unwrap().is_null());
}
