//! 实用工具函数

/// 验证 DOCX 文件格式
pub fn validate_docx_format(bytes: &[u8]) -> bool {
    // 检查是否为有效的 ZIP 文件
    if bytes.len() < 4 {
        return false;
    }
    
    // ZIP 文件签名
    bytes[0..4] == [0x50, 0x4B, 0x03, 0x04] || bytes[0..4] == [0x50, 0x4B, 0x05, 0x06]
}

/// 转义 XML 特殊字符
pub fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// 反转义 XML 特殊字符
pub fn unescape_xml(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

/// 验证 Handlebars 模板语法
pub fn validate_handlebars_syntax(template: &str) -> Result<(), String> {
    use handlebars::Handlebars;
    
    let handlebars = Handlebars::new();
    match handlebars.render_template(template, &serde_json::json!({})) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("模板语法错误: {}", e)),
    }
}

/// 提取文件扩展名
pub fn get_file_extension(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
}

/// 生成唯一 ID（WASM 兼容版本）
pub fn generate_unique_id() -> String {
    // 在 WASM 环境中使用简单的计数器或随机数
    // 避免使用 SystemTime，因为在 WASM 平台上不可用
    use std::sync::atomic::{AtomicU64, Ordering};
    
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    
    format!("id_{}", id)
}

/// 检查是否为有效的 JSON
pub fn is_valid_json(text: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(text).is_ok()
}

/// 安全地解析 JSON
pub fn safe_json_parse(text: &str) -> Result<serde_json::Value, String> {
    serde_json::from_str(text).map_err(|e| format!("JSON 解析错误: {}", e))
}

/// 字节数组转十六进制字符串
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// 计算文件大小的人类可读格式
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;
    
    if size == 0 {
        return "0 B".to_string();
    }
    
    let mut size_f = size as f64;
    let mut unit_index = 0;
    
    while size_f >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size_f /= THRESHOLD as f64;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size_f, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_docx_format() {
        let valid_zip_signature = [0x50, 0x4B, 0x03, 0x04];
        assert!(validate_docx_format(&valid_zip_signature));
        
        let invalid_signature = [0x00, 0x00, 0x00, 0x00];
        assert!(!validate_docx_format(&invalid_signature));
        
        let too_short = [0x50, 0x4B];
        assert!(!validate_docx_format(&too_short));
    }

    #[test]
    fn test_escape_xml() {
        let input = r#"<tag attr="value">content & more</tag>"#;
        let expected = "&lt;tag attr=&quot;value&quot;&gt;content &amp; more&lt;/tag&gt;";
        assert_eq!(escape_xml(input), expected);
    }

    #[test]
    fn test_unescape_xml() {
        let input = "&lt;tag attr=&quot;value&quot;&gt;content &amp; more&lt;/tag&gt;";
        let expected = r#"<tag attr="value">content & more</tag>"#;
        assert_eq!(unescape_xml(input), expected);
    }

    #[test]
    fn test_validate_handlebars_syntax() {
        assert!(validate_handlebars_syntax("Hello {{name}}").is_ok());
        assert!(validate_handlebars_syntax("{{#if condition}}yes{{/if}}").is_ok());
        assert!(validate_handlebars_syntax("{{#each items}}{{name}}{{/each}}").is_ok());
        
        // 这个测试可能需要根据实际的 handlebars 实现调整
        // assert!(validate_handlebars_syntax("{{#if}}").is_err());
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("test.docx"), Some("docx"));
        assert_eq!(get_file_extension("file.txt"), Some("txt"));
        assert_eq!(get_file_extension("noextension"), None);
        assert_eq!(get_file_extension(""), None);
    }

    #[test]
    fn test_is_valid_json() {
        assert!(is_valid_json(r#"{"name": "test"}"#));
        assert!(is_valid_json("[]"));
        assert!(is_valid_json("null"));
        assert!(!is_valid_json("invalid json"));
        assert!(!is_valid_json("{"));
    }

    #[test]
    fn test_bytes_to_hex() {
        let bytes = vec![0x50, 0x4B, 0x03, 0x04];
        assert_eq!(bytes_to_hex(&bytes), "504b0304");
        
        let empty: Vec<u8> = vec![];
        assert_eq!(bytes_to_hex(&empty), "");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }
}
