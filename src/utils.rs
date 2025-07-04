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

/// XML 转义符转换成正常字符
pub fn xml_escape_to_normal(xml_content: String) -> String {
    xml_content
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

/// 合并被XML标签分割的Handlebars语法
/// 这个函数会识别被分割的 Handlebars 表达式并将其合并
pub fn merge_handlebars_in_xml(xml_content: String) -> Result<String, Box<dyn std::error::Error>> {
    
    // 判断如果 xml_content 中没有 { 字符, 则直接返回 xml_content 本身
    if !xml_content.contains('{') {
        return Ok(xml_content);
    }
    
    let len = xml_content.len();
    let chars = xml_content.chars();
    
    // 是否在尖括号中间
    let mut is_in_angle = false;
    
    // 尖括号中间的内容
    let mut angle_content = vec![];
    // 尖括号外面的内容
    let mut text_content = vec![];
    
    let mut contents: Vec<(String, u8)> = vec![];
    
    for c in chars.into_iter() {
        
        if c == '<' {
            is_in_angle = true;
            angle_content.push(c);
            if text_content.len() > 0 {
                contents.push((xml_escape_to_normal(text_content.iter().collect::<String>()), 0_u8));
                text_content = vec![];
            }
            continue;
        }
        
        if c == '>' {
            angle_content.push(c);
            is_in_angle = false;
            contents.push((angle_content.iter().collect::<String>(), 1_u8));
            angle_content = vec![];
            continue;
        }
        
        if is_in_angle {
            angle_content.push(c);
            continue;
        }
        
        text_content.push(c);
        
    }
    
    let mut result: String = String::with_capacity(len);
    
    // 大括号的数量
    let mut in_braces_num: i32 = 0;
    
    for (i, (part, ty)) in contents
      .iter()
      .enumerate()
    {
        if *ty == 0_u8 {
            // 如果上一个是开始标签, 下一个也是开始标签, 忽略
            // 上一个是结束标签, 下一个也是结束标签, 忽略
            if i > 0 && i < contents.len() - 1 && contents[i - 1].1 == 1_u8 && contents[i + 1].1 == 1_u8 {
                if !contents[i - 1].0.starts_with("</") && !contents[i + 1].0.starts_with("</") {
                    continue;
                }
                if contents[i - 1].0.starts_with("</") && contents[i + 1].0.starts_with("</") {
                    continue;
                }
                // 如果上一个标签名称和下一个标签名称剔除掉 < 跟 </ 之后不相同, 忽略
                
                let prev_tag_name = contents[i - 1].0
                  .trim_start_matches('<')
                  .trim_end_matches('>')
                  .split_whitespace()
                  .next()
                  .unwrap_or("");
                
                let next_tag_name = contents[i + 1].0
                  .trim_start_matches('<')
                  .trim_end_matches('>')
                  .split_whitespace()
                  .next()
                  .unwrap_or("");
                
                if prev_tag_name == next_tag_name || prev_tag_name != next_tag_name.trim_start_matches('/') {
                    continue;
                }
            }
            
            for c in part.chars() {
                if c == '{' {
                    in_braces_num += 1;
                } else if c == '}' {
                    in_braces_num -= 1;
                }
            }
            result.push_str(&part);
            continue;
        }
        if in_braces_num == 0 {
            result.push_str(&part);
            continue;
        }
    }
    
    Ok(result)
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
            Err(format!("Handlebars 模板渲染失败: {e:#?}").into())
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


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs::File;
//     use std::io::Write;
    
//     #[test]
//     fn test_merge_handlebars_in_xml() {
//         let mut file = File::open("examples/document.xml").unwrap();
//         let mut xml_content = String::new();
//         file.read_to_string(&mut xml_content).unwrap();
//         let result = merge_handlebars_in_xml(xml_content).unwrap();
//         let mut file = File::create("examples/document_output.xml").unwrap();
//         file.write_all(result.as_bytes()).unwrap();
//     }
// }
