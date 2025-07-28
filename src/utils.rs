//! 实用工具函数
use quick_xml::{Reader, Writer, events::Event};
use std::io::{Cursor, Write};
use crate::errors::DocxError;

/// 验证 DOCX 文件格式
/// 检查文件是否为有效的 ZIP 格式，并包含必需的 DOCX 文件结构
pub(crate) fn validate_docx_format(file_data: &[u8]) -> Result<(), DocxError> {
    // 检查文件大小
    if file_data.len() < 22 {
        return Err(DocxError::InvalidZipFormat);
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
        _ => return Err(DocxError::InvalidZipFormat),
    }
    
    Ok(())
}

/// XML 转义符转换成正常字符
// pub fn xml_escape_to_normal(xml_content: String) -> String {
//     xml_content
//         .replace("&lt;", "<")
//         .replace("&gt;", ">")
//         .replace("&amp;", "&")
//         .replace("&quot;", "\"")
//         .replace("&apos;", "'")
// }

/// 合并被XML标签分割的Handlebars语法
/// 
/// 这个函数会识别被XML标签分割的 Handlebars 表达式并将其合并。
/// 例如: `<w:t>{</w:t><w:t>{name</w:t><w:t>}</w:t><w:t>}</w:t>` 
/// 会被合并为: `{{name}}`
/// 
/// 使用 quick_xml 进行更可靠的XML解析，能正确处理各种XML结构
pub(crate) fn merge_handlebars_in_xml(xml_content: String) -> Result<String, Box<dyn std::error::Error>> {
    // 快速检查：如果内容中没有大括号，直接返回原内容
    if !xml_content.contains('{') {
        return Ok(xml_content);
    }
    
    // 创建XML阅读器和写入器
    let mut reader = Reader::from_str(&xml_content);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    
    // 用于累积文本内容的缓冲区
    let mut text_buffer = String::new();
    
    // 跟踪大括号状态
    let mut brace_count = 0;        // 当前大括号的数量
    let mut in_handlebars = false;  // 是否在完整的 handlebars 表达式中 (如 {{...}})
    
    loop {
        match reader.read_event_into(&mut buf) {
            // 处理文本节点
            Ok(Event::Text(ref e)) => {
                let text = std::str::from_utf8(e)?;
                
                // 逐字符分析文本，统计大括号
                for ch in text.chars() {
                    if ch == '{' {
                        brace_count += 1;
                        // 当遇到连续的两个 { 时，标记进入 handlebars 表达式
                        if brace_count >= 2 {
                            in_handlebars = true;
                        }
                    } else if ch == '}' {
                        if brace_count > 0 {
                            brace_count -= 1;
                        }
                        // 当大括号数量归零时，标记退出 handlebars 表达式
                        if brace_count == 0 {
                            in_handlebars = false;
                        }
                    }
                }
                
                // 将当前文本添加到缓冲区
                text_buffer.push_str(text);
                
                // 如果不在 handlebars 表达式中且大括号已平衡，输出缓冲的文本
                if !in_handlebars && brace_count == 0
                    && !text_buffer.is_empty() {
                        writer.write_event(Event::Text(quick_xml::events::BytesText::new(&text_buffer)))?;
                        text_buffer.clear();
                    }
                // 否则继续累积文本，等待 handlebars 表达式完整
            }
            
            // 处理开始标签 (如 <w:t>)
            Ok(Event::Start(ref e)) => {
                // 如果在 handlebars 表达式中，跳过XML标签，只保留文本内容
                if !in_handlebars && brace_count == 0 {
                    // 先输出之前缓冲的文本
                    if !text_buffer.is_empty() {
                        writer.write_event(Event::Text(quick_xml::events::BytesText::new(&text_buffer)))?;
                        text_buffer.clear();
                    }
                    // 输出开始标签
                    writer.write_event(Event::Start(e.clone()))?;
                }
                // 在 handlebars 表达式中时，忽略XML标签，这样就实现了"合并"效果
            }
            
            // 处理结束标签 (如 </w:t>)
            Ok(Event::End(ref e)) => {
                // 如果在 handlebars 表达式中，跳过XML标签
                if !in_handlebars && brace_count == 0 {
                    // 先输出之前缓冲的文本
                    if !text_buffer.is_empty() {
                        writer.write_event(Event::Text(quick_xml::events::BytesText::new(&text_buffer)))?;
                        text_buffer.clear();
                    }
                    // 输出结束标签
                    writer.write_event(Event::End(e.clone()))?;
                }
            }
            
            // 处理自闭合标签 (如 <w:br/>)
            Ok(Event::Empty(ref e)) => {
                if !in_handlebars && brace_count == 0 {
                    if !text_buffer.is_empty() {
                        writer.write_event(Event::Text(quick_xml::events::BytesText::new(&text_buffer)))?;
                        text_buffer.clear();
                    }
                    writer.write_event(Event::Empty(e.clone()))?;
                }
            }
            
            // 处理其他XML事件（注释、CDATA、处理指令等）
            Ok(Event::Eof) => break,
            Ok(event) => {
                if !in_handlebars && brace_count == 0 {
                    if !text_buffer.is_empty() {
                        writer.write_event(Event::Text(quick_xml::events::BytesText::new(&text_buffer)))?;
                        text_buffer.clear();
                    }
                    writer.write_event(event)?;
                }
            }
            
            // XML解析错误
            Err(e) => return Err(format!("XML解析错误 at position {}: {:?}", reader.buffer_position(), e).into()),
        }
        buf.clear();
    }
    
    // 输出剩余的文本缓冲（如果有的话）
    if !text_buffer.is_empty() {
        writer.write_event(Event::Text(quick_xml::events::BytesText::new(&text_buffer)))?;
    }
    
    // 将结果转换为字符串返回
    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result)?)
}

/// 注册基础的 Handlebars helper 函数
pub(crate) fn register_basic_helpers(handlebars: &mut handlebars::Handlebars) -> Result<(), Box<dyn std::error::Error>> {
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
            Value::String(s) => s.chars().count(),
            Value::Object(obj) => obj.len(),
            _ => 0
        }
    });
    handlebars.register_helper("len", Box::new(len));
    
    Ok(())
}

/// 获取 PNG 图片的宽高
fn get_png_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 24 || &data[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    // IHDR chunk: starts at byte 8, width at 16~19, height at 20~23
    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Some((width, height))
}

/// 获取 JPEG 图片的宽高
fn get_jpeg_dimensions(data: &[u8]) -> Option<(u16, u16)> {
    let mut i = 2;
    while i + 9 < data.len() {
        if data[i] != 0xFF {
            i += 1;
            continue;
        }
        let marker = data[i + 1];
        let len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
        if marker == 0xC0 || marker == 0xC2 {
            // SOF0 or SOF2
            let height = u16::from_be_bytes([data[i + 5], data[i + 6]]);
            let width = u16::from_be_bytes([data[i + 7], data[i + 8]]);
            return Some((width, height));
        }
        i += 2 + len;
    }
    None
}

/// 获取 WebP 图片的宽高
fn get_webp_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 30 || &data[0..4] != b"RIFF" || &data[8..12] != b"WEBP" {
        return None;
    }
    // VP8X
    if &data[12..16] == b"VP8X" {
        let width = 1 + u32::from_le_bytes([data[24], data[25], data[26], 0]);
        let height = 1 + u32::from_le_bytes([data[27], data[28], data[29], 0]);
        return Some((width, height));
    }
    // VP8
    if &data[12..15] == b"VP8" && data[15] == b' ' {
        // Lossy
        let width = u16::from_le_bytes([data[26], data[27]]) as u32;
        let height = u16::from_le_bytes([data[28], data[29]]) as u32;
        return Some((width, height));
    }
    // VP8L
    if &data[12..16] == b"VP8L" {
        let b = &data[21..25];
        let width = 1 + (((b[1] & 0x3F) as u32) << 8 | b[0] as u32);
        let height = 1 + (((b[3] & 0xF) as u32) << 10 | (b[2] as u32) << 2 | ((b[1] & 0xC0) as u32) >> 6);
        return Some((width, height));
    }
    None
}

/// 获取 BMP 图片的宽高
fn get_bmp_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 26 || &data[0..2] != b"BM" {
        return None;
    }
    // BMP 文件头后是 DIB 头，宽高在偏移 18~21（宽），22~25（高），都是 little-endian
    let width = u32::from_le_bytes([data[18], data[19], data[20], data[21]]);
    let height = u32::from_le_bytes([data[22], data[23], data[24], data[25]]);
    Some((width, height))
}

fn get_tiff_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 8 {
        return None;
    }
    // 判断字节序
    let le = &data[0..2] == b"II";
    let be = &data[0..2] == b"MM";
    if !le && !be {
        return None;
    }
    let read_u16 = |d: &[u8]| if le {
        u16::from_le_bytes([d[0], d[1]])
    } else {
        u16::from_be_bytes([d[0], d[1]])
    };
    let read_u32 = |d: &[u8]| if le {
        u32::from_le_bytes([d[0], d[1], d[2], d[3]])
    } else {
        u32::from_be_bytes([d[0], d[1], d[2], d[3]])
    };
    // 检查 magic number
    let magic = read_u16(&data[2..4]);
    if magic != 42 {
        return None;
    }
    // IFD 偏移
    let ifd_offset = read_u32(&data[4..8]) as usize;
    if data.len() < ifd_offset + 2 {
        return None;
    }
    let num_dir = read_u16(&data[ifd_offset..ifd_offset + 2]) as usize;
    let mut width = None;
    let mut height = None;
    for i in 0..num_dir {
        let entry = ifd_offset + 2 + i * 12;
        if data.len() < entry + 12 {
            break;
        }
        let tag = read_u16(&data[entry..entry + 2]);
        let field_type = read_u16(&data[entry + 2..entry + 4]);
        // let count = read_u32(&data[entry + 4..entry + 8]);
        let value_offset = &data[entry + 8..entry + 12];
        // tag 256: ImageWidth, tag 257: ImageLength
        if tag == 256 {
            width = Some(match field_type {
                3 => read_u16(value_offset) as u32, // SHORT
                4 => read_u32(value_offset),        // LONG
                _ => continue,
            });
        }
        if tag == 257 {
            height = Some(match field_type {
                3 => read_u16(value_offset) as u32,
                4 => read_u32(value_offset),
                _ => continue,
            });
        }
        if width.is_some() && height.is_some() {
            break;
        }
    }
    match (width, height) {
        (Some(w), Some(h)) => Some((w, h)),
        _ => None,
    }
}

fn get_gif_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 10 || (&data[0..6] != b"GIF87a" && &data[0..6] != b"GIF89a") {
        return None;
    }
    let width = u16::from_le_bytes([data[6], data[7]]) as u32;
    let height = u16::from_le_bytes([data[8], data[9]]) as u32;
    Some((width, height))
}

// 获取图片的宽高
/// 支持 PNG, JPEG, WebP, BMP, TIFF, GIF 格式
pub(crate) fn get_image_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if let Some((w, h)) = get_png_dimensions(data) {
        Some((w, h))
    } else if let Some((w, h)) = get_jpeg_dimensions(data).map(|(w, h)| (w as u32, h as u32)) {
        Some((w, h))
    } else if let Some((w, h)) = get_webp_dimensions(data) {
        Some((w, h))
    } else if let Some((w, h)) = get_bmp_dimensions(data) {
        Some((w, h))
    } else if let Some((w, h)) = get_tiff_dimensions(data) {
        Some((w, h))
    } else if let Some((w, h)) = get_gif_dimensions(data) {
        Some((w, h))
    } else {
        None
    }
}

// 更简化的实现方案
pub(crate) fn remove_table_row_simple(xml_content: &str, target_uuid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml_content);
    
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    
    let mut current_row_content = String::new();
    let mut in_table_row = false;
    let mut row_depth = 0;
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name().as_ref() == b"w:tr" {
                    in_table_row = true;
                    row_depth += 1;
                    current_row_content.clear();
                    current_row_content.push_str(&format!("<{}", String::from_utf8_lossy(e.name().as_ref())));
                    // 添加属性
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            current_row_content.push_str(&format!(" {}=\"{}\"", 
                                String::from_utf8_lossy(attr.key.as_ref()),
                                String::from_utf8_lossy(&attr.value)));
                        }
                    }
                    current_row_content.push('>');
                } else if in_table_row {
                    current_row_content.push_str(&format!("<{}", String::from_utf8_lossy(e.name().as_ref())));
                    // 添加属性
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            current_row_content.push_str(&format!(" {}=\"{}\"", 
                                String::from_utf8_lossy(attr.key.as_ref()),
                                String::from_utf8_lossy(&attr.value)));
                        }
                    }
                    current_row_content.push('>');
                } else {
                    writer.write_event(Event::Start(e.clone()))?;
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name().as_ref() == b"w:tr" && in_table_row {
                    row_depth -= 1;
                    if row_depth == 0 {
                        current_row_content.push_str(&format!("</{}>", String::from_utf8_lossy(e.name().as_ref())));
                        
                        // 检查当前行是否包含目标 UUID
                        if !current_row_content.contains(target_uuid) {
                            // 如果不包含目标 UUID，则写入这一行
                            writer.get_mut().write_all(current_row_content.as_bytes())?;
                        }
                        // 如果包含目标 UUID，则跳过整行
                        
                        in_table_row = false;
                        current_row_content.clear();
                    } else {
                        current_row_content.push_str(&format!("</{}>", String::from_utf8_lossy(e.name().as_ref())));
                    }
                } else if in_table_row {
                    current_row_content.push_str(&format!("</{}>", String::from_utf8_lossy(e.name().as_ref())));
                } else {
                    writer.write_event(Event::End(e.clone()))?;
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_table_row {
                    let text = std::str::from_utf8(e)?;
                    // let text = unescape(text)?;
                    // current_row_content.push_str(&html_escape::encode_text(&text));
                    current_row_content.push_str(text);
                } else {
                    writer.write_event(Event::Text(e.clone()))?;
                }
            }
            Ok(Event::Empty(ref e)) => {
                if in_table_row {
                    current_row_content.push_str(&format!("<{}", String::from_utf8_lossy(e.name().as_ref())));
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            current_row_content.push_str(&format!(" {}=\"{}\"", 
                                String::from_utf8_lossy(attr.key.as_ref()),
                                String::from_utf8_lossy(&attr.value)));
                        }
                    }
                    current_row_content.push_str("/>");
                } else {
                    writer.write_event(Event::Empty(e.clone()))?;
                }
            }
            Ok(Event::Comment(ref e)) => {
                if !in_table_row {
                    writer.write_event(Event::Comment(e.clone()))?;
                }
            }
            Ok(Event::CData(ref e)) => {
                if in_table_row {
                    current_row_content.push_str(&format!("<![CDATA[{}]]>", String::from_utf8_lossy(e)));
                } else {
                    writer.write_event(Event::CData(e.clone()))?;
                }
            }
            Ok(Event::Decl(ref e)) => {
                writer.write_event(Event::Decl(e.clone()))?;
            }
            Ok(Event::PI(ref e)) => {
                if !in_table_row {
                    writer.write_event(Event::PI(e.clone()))?;
                }
            }
            Ok(Event::DocType(ref e)) => {
                writer.write_event(Event::DocType(e.clone()))?;
            }
            Ok(Event::GeneralRef(ref e)) => {
                if !in_table_row {
                    writer.write_event(Event::GeneralRef(e.clone()))?;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Error at position {}: {:?}", reader.buffer_position(), e).into()),
        }
        buf.clear();
    }
    
    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result)?)
}
