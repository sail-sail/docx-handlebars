//! 实用工具函数

use std::io::Read;
use crate::errors::DocxError;

/// 验证 DOCX 文件格式
/// 检查文件是否为有效的 ZIP 格式，并包含必需的 DOCX 文件结构
pub fn validate_docx_format(file_data: &[u8]) -> Result<(), DocxError> {
    // 检查文件大小
    if file_data.len() < 22 {
        return Err(DocxError::FileTooSmall);
    }
    
    // 检查 ZIP 文件签名
    // ZIP 文件的签名通常是 0x504B0304 (PK..) 或 0x504B0506 (PK.. 空文件)
    // 或者 0x504B0708 (PK.. 分割压缩包)
    let signature = u32::from_le_bytes([
        file_data[0], file_data[1], file_data[2], file_data[3]
    ]);
    
    match signature {
        0x04034b50 | 0x06054b50 | 0x08074b50 => {
            // 有效的 ZIP 签名
        },
        _ => return Err(DocxError::InvalidZipSignature),
    }
    
    // 尝试解析 ZIP 文件并检查必需的 DOCX 结构
    validate_docx_structure(file_data)?;
    
    Ok(())
}

/// 验证 DOCX 文件的内部结构
fn validate_docx_structure(file_data: &[u8]) -> Result<(), DocxError> {
    use std::io::Cursor;
    use zip::ZipArchive;
    
    let cursor = Cursor::new(file_data);
    let mut zip = match ZipArchive::new(cursor) {
        Ok(zip) => zip,
        Err(e) => return Err(DocxError::ZipReadError(format!("无法读取 ZIP 文件: {}", e))),
    };
    
    // 检查必需的 DOCX 文件
    let required_files = vec![
        "[Content_Types].xml",
        "_rels/.rels",
        "word/document.xml",
    ];
    
    for required_file in required_files {
        if zip.by_name(required_file).is_err() {
            return Err(DocxError::MissingRequiredFile(required_file.to_string()));
        }
    }
    
    // 可选：验证 Content_Types.xml 包含正确的 MIME 类型
    if let Ok(mut content_types) = zip.by_name("[Content_Types].xml") {
        let mut content = String::new();
        if content_types.read_to_string(&mut content).is_ok() {
            if !content.contains("application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml") {
                return Err(DocxError::InvalidZipFormat);
            }
        }
    }
    
    Ok(())
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

/// 应用 Handlebars 模板引擎处理
/// 将合并后的 XML 内容中的 Handlebars 语法替换为实际数据
pub fn apply_handlebars_template(
    template_content: &str,
    data: &serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
    use handlebars::Handlebars;
    
    // 创建 Handlebars 引擎实例
    let mut handlebars = Handlebars::new();
    
    // 配置 Handlebars
    handlebars.set_strict_mode(false); // 允许未定义的变量
    
    // 注册一些常用的 helper 函数
    register_basic_helpers(&mut handlebars)?;
    
    // 渲染模板
    match handlebars.render_template(template_content, data) {
        Ok(result) => Ok(result),
        Err(e) => {
            // 如果渲染失败，提供更详细的错误信息
            Err(format!("Handlebars 模板渲染失败: {}", e).into())
        }
    }
}

/// 注册基础的 Handlebars helper 函数
fn register_basic_helpers(handlebars: &mut handlebars::Handlebars) -> Result<(), Box<dyn std::error::Error>> {
    use handlebars::handlebars_helper;
    use serde_json::Value;
    
    // 注册 eq helper (相等比较)
    handlebars_helper!(eq: |x: Value, y: Value| x == y);
    handlebars.register_helper("eq", Box::new(eq));
    
    // 注册 ne helper (不等比较)  
    handlebars_helper!(ne: |x: Value, y: Value| x != y);
    handlebars.register_helper("ne", Box::new(ne));
    
    // 注册 gt helper (大于)
    handlebars_helper!(gt: |x: i64, y: i64| x > y);
    handlebars.register_helper("gt", Box::new(gt));
    
    // 注册 lt helper (小于)
    handlebars_helper!(lt: |x: i64, y: i64| x < y);
    handlebars.register_helper("lt", Box::new(lt));
    
    // 注册 upper helper (转大写)
    handlebars_helper!(upper: |s: String| s.to_uppercase());
    handlebars.register_helper("upper", Box::new(upper));
    
    // 注册 lower helper (转小写)
    handlebars_helper!(lower: |s: String| s.to_lowercase());
    handlebars.register_helper("lower", Box::new(lower));
    
    // 注册 len helper (数组/字符串长度)
    handlebars_helper!(len: |x: Value| {
        match x {
            Value::Array(arr) => arr.len(),
            Value::String(s) => s.len(),
            Value::Object(obj) => obj.len(),
            _ => 0
        }
    });
    handlebars.register_helper("len", Box::new(len));
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
    
    #[test]
    fn test_apply_handlebars_template() {
        use serde_json::json;
        
        // 测试简单变量替换
        let template = "Hello {{name}}, you are {{age}} years old.";
        let data = json!({
            "name": "张三",
            "age": 25
        });
        
        let result = apply_handlebars_template(template, &data).unwrap();
        assert_eq!(result, "Hello 张三, you are 25 years old.");
        
        // 测试条件渲染
        let conditional_template = "{{#if has_bonus}}奖金: ¥{{bonus_amount}}{{/if}}";
        let data_with_bonus = json!({
            "has_bonus": true,
            "bonus_amount": 5000
        });
        
        let result_with_bonus = apply_handlebars_template(conditional_template, &data_with_bonus).unwrap();
        assert_eq!(result_with_bonus, "奖金: ¥5000");
        
        let data_no_bonus = json!({
            "has_bonus": false,
            "bonus_amount": 5000
        });
        
        let result_no_bonus = apply_handlebars_template(conditional_template, &data_no_bonus).unwrap();
        assert_eq!(result_no_bonus, "");
        
        // 测试循环渲染
        let loop_template = "项目列表:{{#each projects}} - {{name}}{{/each}}";
        let data_with_projects = json!({
            "projects": [
                {"name": "项目A"},
                {"name": "项目B"},
                {"name": "项目C"}
            ]
        });
        
        let result_projects = apply_handlebars_template(loop_template, &data_with_projects).unwrap();
        assert_eq!(result_projects, "项目列表: - 项目A - 项目B - 项目C");
    }
    
    #[test]
    fn test_handlebars_helpers() {
        use serde_json::json;
        
        // 测试 eq helper
        let eq_template = "{{#if (eq status \"completed\")}}已完成{{else}}未完成{{/if}}";
        let data = json!({"status": "completed"});
        let result = apply_handlebars_template(eq_template, &data).unwrap();
        assert_eq!(result, "已完成");
        
        // 测试 upper helper
        let upper_template = "{{upper name}}";
        let data = json!({"name": "hello world"});
        let result = apply_handlebars_template(upper_template, &data).unwrap();
        assert_eq!(result, "HELLO WORLD");
        
        // 测试 len helper
        let len_template = "共有 {{len items}} 项";
        let data = json!({"items": ["a", "b", "c"]});
        let result = apply_handlebars_template(len_template, &data).unwrap();
        assert_eq!(result, "共有 3 项");
    }
    
    #[test]
    fn test_complete_template_processing() {
        use serde_json::json;
        
        // 测试简单的完整流程
        let simple_split = r#"<w:t>员工姓名: {{</w:t><w:t>employee.name</w:t><w:t>}}</w:t>"#;
        
        let data = json!({
            "employee": {
                "name": "张三",
                "department": "技术部"
            }
        });
        
        // 第一步：合并被分割的 Handlebars 语法
        let merged = merge_handlebars_in_xml(simple_split).unwrap();
        println!("合并后: {}", merged);
        assert_eq!(merged, r#"<w:t>员工姓名: {{employee.name}}</w:t>"#);
        
        // 第二步：应用 Handlebars 模板渲染
        let rendered = apply_handlebars_template(&merged, &data).unwrap();
        println!("渲染后: {}", rendered);
        assert_eq!(rendered, r#"<w:t>员工姓名: 张三</w:t>"#);
        
        // 测试更复杂的条件渲染
        let conditional_template = "{{#if employee.has_bonus}}奖金: ¥{{employee.bonus_amount}}{{/if}}";
        let data_with_bonus = json!({
            "employee": {
                "has_bonus": true,
                "bonus_amount": 5000
            }
        });
        
        let rendered_conditional = apply_handlebars_template(conditional_template, &data_with_bonus).unwrap();
        println!("条件渲染: {}", rendered_conditional);
        assert_eq!(rendered_conditional, "奖金: ¥5000");
    }
    
    #[test]
    fn test_validate_docx_format() {
        // 测试文件太小的情况
        let small_file = vec![0u8; 10];
        let result = validate_docx_format(&small_file);
        assert!(matches!(result, Err(DocxError::FileTooSmall)));
        
        // 测试无效的 ZIP 签名
        let invalid_file = vec![0u8; 100];
        let result = validate_docx_format(&invalid_file);
        assert!(matches!(result, Err(DocxError::InvalidZipSignature)));
        
        // 测试有效的 ZIP 签名但不是有效的 ZIP 文件
        let mut fake_zip = vec![0x50, 0x4B, 0x03, 0x04]; // 有效的 ZIP 签名
        fake_zip.extend(vec![0u8; 100]); // 添加一些无效的数据
        let result = validate_docx_format(&fake_zip);
        assert!(matches!(result, Err(DocxError::ZipReadError(_))));
    }
}