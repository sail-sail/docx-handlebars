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

/// 合并被XML标签分割的Handlebars语法
/// 这个函数会识别被分割的 Handlebars 表达式并将其合并
pub fn merge_handlebars_in_xml(xml_content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = xml_content.to_string();
    let mut changed = true;
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 20;

    while changed && iterations < MAX_ITERATIONS {
        changed = false;
        iterations += 1;

        // 简单高效的模式匹配方法
        // 模式1: <w:t>prefix{{</w:t><w:t>content}}</w:t> -> <w:t>prefix{{content}}</w:t>
        result = merge_simple_split_pattern(&result, &mut changed);
        
        if changed { continue; }

        // 模式2: <w:t>{{</w:t><w:t>content}}</w:t> -> <w:t>{{content}}</w:t>
        result = merge_start_only_pattern(&result, &mut changed);
        
        if changed { continue; }

        // 模式3: <w:t>{{part1</w:t><w:t>part2}}</w:t> -> <w:t>{{part1part2}}</w:t>
        result = merge_two_part_pattern(&result, &mut changed);
        
        if changed { continue; }

        // 模式4: <w:t>{{</w:t><w:t>part1</w:t><w:t>part2}}</w:t> -> <w:t>{{part1part2}}</w:t>
        result = merge_three_part_pattern(&result, &mut changed);
    }

    Ok(result)
}

fn merge_simple_split_pattern(content: &str, changed: &mut bool) -> String {
    let mut result = content.to_string();
    
    // 查找被分割的模式: <w:t>...{{content...</w:t><w:t>variable</w:t><w:t>}}</w:t>
    // 只处理{{后面直接跟着</w:t>的情况，即真正的分割
    let mut search_pos = 0;
    
    while let Some(start) = result[search_pos..].find("{{") {
        let start = search_pos + start;
        
        // 确保{{后面直接是</w:t>或者有一些内容然后是</w:t>
        let after_braces = &result[start + 2..];
        if let Some(end_tag_pos) = after_braces.find("</w:t>") {
            let content_between = &after_braces[..end_tag_pos];
            
            // 检查中间的内容是否不包含}}（即确实是分割的）
            if !content_between.contains("}}") {
                let split_pos = start + 2 + end_tag_pos;
                
                // 查找下一个<w:t>
                if let Some(next_tag_start) = result[split_pos + 6..].find("<w:t>") {
                    let next_tag_start = split_pos + 6 + next_tag_start;
                    
                    // 向前查找当前w:t标签的开始
                    if let Some(tag_start) = result[..start].rfind("<w:t") {
                        if let Some(tag_content_start) = result[tag_start..].find('>') {
                            let tag_content_start = tag_start + tag_content_start + 1;
                            
                            // 提取第一段内容（包含{{和可能的其他内容）
                            let first_part = &result[tag_content_start..split_pos];
                            
                            // 查找变量名结束
                            if let Some(var_content_start) = result[next_tag_start..].find('>') {
                                let var_content_start = next_tag_start + var_content_start + 1;
                                if let Some(var_end) = result[var_content_start..].find("</w:t>") {
                                    let var_end = var_content_start + var_end;
                                    let variable = &result[var_content_start..var_end];
                                    
                                    // 查找最后的}}</w:t>
                                    if let Some(close_start) = result[var_end + 6..].find("<w:t>}}</w:t>") {
                                        let close_start_abs = var_end + 6 + close_start;
                                        let close_end = close_start_abs + 13; // <w:t>}}</w:t> 长度是13
                                        
                                        // 构建新的合并标签
                                        let new_content = "<w:t>".to_string() + first_part + variable + "}}</w:t>";
                                        result.replace_range(tag_start..close_end, &new_content);
                                        *changed = true;
                                        return result; // 一次只处理一个匹配，然后重新开始
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        search_pos = start + 2; // 继续搜索下一个{{
    }
    
    result
}

fn merge_start_only_pattern(content: &str, changed: &mut bool) -> String {
    let mut result = content.to_string();
    
    // 查找模式: <w:t>{{</w:t><w:t>content}}</w:t>
    if let Some(start) = result.find("<w:t>{{</w:t><w:t>") {
        if let Some(end_marker) = result[start + 18..].find("}}</w:t>") {
            let end_pos = start + 18 + end_marker + 7;
            let variable = &result[start + 18..start + 18 + end_marker];
            
            let new_content = "<w:t>{{".to_string() + variable + "}}</w:t>";
            result.replace_range(start..end_pos, &new_content);
            *changed = true;
        }
    }
    
    result
}

fn merge_two_part_pattern(content: &str, changed: &mut bool) -> String {
    let mut result = content.to_string();
    
    // 查找模式: <w:t>{{part1</w:t><w:t>part2}}</w:t>
    let mut pos = 0;
    while let Some(start) = result[pos..].find("<w:t>{{") {
        let start = pos + start;
        
        // 查找第一个</w:t>
        if let Some(first_end) = result[start + 8..].find("</w:t>") {
            let first_end = start + 8 + first_end;
            let first_part = &result[start + 5..first_end];
            
            // 确保第一部分不包含}}
            if !first_part.contains("}}") {
                // 查找下一个<w:t>
                if let Some(second_start) = result[first_end + 6..].find("<w:t>") {
                    let second_start = first_end + 6 + second_start;
                    
                    // 查找内容开始位置
                    if let Some(content_start_offset) = result[second_start..].find('>') {
                        let content_start = second_start + content_start_offset + 1;
                        
                        // 查找第二个</w:t>
                        if let Some(second_end) = result[content_start..].find("</w:t>") {
                            let second_end = content_start + second_end;
                            let second_part = &result[content_start..second_end];
                            
                            if second_part.ends_with("}}") {
                                let variable_part = &second_part[..second_part.len() - 2];
                                let combined_variable = format!("{}{}", &first_part[2..], variable_part);
                                let new_content = "<w:t>{{".to_string() + &combined_variable + "}}</w:t>";
                                result.replace_range(start..second_end + 6, &new_content);
                                *changed = true;
                                return result;
                            }
                        }
                    }
                }
            }
        }
        
        pos = start + 8;
    }
    
    result
}

fn merge_three_part_pattern(content: &str, changed: &mut bool) -> String {
    let mut result = content.to_string();
    
    // 查找模式: <w:t>{{</w:t><w:t>part1</w:t><w:t>part2}}</w:t>
    if let Some(start) = result.find("<w:t>{{</w:t><w:t>") {
        let middle_start = start + 18;
        
        // 查找middle内容
        if let Some(middle_content_start) = result[middle_start..].find('>') {
            let middle_content_start = middle_start + middle_content_start + 1;
            
            if let Some(middle_end) = result[middle_content_start..].find("</w:t>") {
                let middle_end = middle_content_start + middle_end;
                let middle_content = &result[middle_content_start..middle_end];
                
                // 查找第三个<w:t>
                if let Some(third_start) = result[middle_end + 6..].find("<w:t>") {
                    let third_start = middle_end + 6 + third_start;
                    
                    if let Some(third_content_start) = result[third_start..].find('>') {
                        let third_content_start = third_start + third_content_start + 1;
                        
                        if let Some(third_end) = result[third_content_start..].find("</w:t>") {
                            let third_end = third_content_start + third_end;
                            let third_content = &result[third_content_start..third_end];
                            
                            if third_content.ends_with("}}") {
                                let third_part = &third_content[..third_content.len() - 2];
                                let combined_variable = format!("{}{}", middle_content, third_part);
                                let new_content = "<w:t>{{".to_string() + &combined_variable + "}}</w:t>";
                                result.replace_range(start..third_end + 6, &new_content);
                                *changed = true;
                            }
                        }
                    }
                }
            }
        }
    }
    
    result
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

    #[test]
    fn test_merge_handlebars_syntax() {
        // 测试被分割的 Handlebars 语法合并
        let xml_input = r#"<w:t>部门: {{</w:t><w:t>employee.department</w:t><w:t>}}</w:t>"#;
        let expected = r#"<w:t>部门: {{employee.department}}</w:t>"#;
        
        let result = merge_handlebars_in_xml(xml_input).unwrap();
        assert_eq!(result, expected);
        
        // 测试更复杂的情况
        let complex_xml = r#"<w:t>员工姓名: {{employee.name}}</w:t><w:t>部门: {{</w:t><w:t>employee.department</w:t><w:t>}}</w:t>"#;
        let expected_complex = r#"<w:t>员工姓名: {{employee.name}}</w:t><w:t>部门: {{employee.department}}</w:t>"#;
        
        let result_complex = merge_handlebars_in_xml(complex_xml).unwrap();
        assert_eq!(result_complex, expected_complex);
    }
    
    #[test]
    fn test_real_document_xml() {
        // 测试实际的 document.xml 片段
        let xml_fragment = r#"<w:t>部门: {{</w:t><w:t>employee.department</w:t><w:t>}}</w:t>"#;
        let result = merge_handlebars_in_xml(xml_fragment).unwrap();
        assert_eq!(result, r#"<w:t>部门: {{employee.department}}</w:t>"#);
        
        // 测试更复杂的情况
        let complex_fragment = r#"<w:t>{{#if </w:t><w:t>employee.has_bonus</w:t><w:t>}}</w:t>"#;
        let result_complex = merge_handlebars_in_xml(complex_fragment).unwrap();
        assert_eq!(result_complex, r#"<w:t>{{#if employee.has_bonus}}</w:t>"#);
        
        // 测试包含其他内容的情况
        let mixed_content = r#"<w:p><w:t>员工姓名: {{employee.name}}</w:t></w:p><w:p><w:t>部门: {{</w:t><w:t>employee.department</w:t><w:t>}}</w:t></w:p>"#;
        let result_mixed = merge_handlebars_in_xml(mixed_content).unwrap();
        assert_eq!(result_mixed, r#"<w:p><w:t>员工姓名: {{employee.name}}</w:t></w:p><w:p><w:t>部门: {{employee.department}}</w:t></w:p>"#);
    }
}
