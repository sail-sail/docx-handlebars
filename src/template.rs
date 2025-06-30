use serde_json::Value;
use std::io::{Cursor, Read, Write};
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};
use std::collections::HashMap;
use crate::utils::{merge_handlebars_in_xml, apply_handlebars_template, validate_docx_format};

pub fn render_handlebars(
  zip_bytes: Vec<u8>,
  data: &Value,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
  // 首先验证输入是否为有效的 DOCX 文件
  validate_docx_format(&zip_bytes)?;
  
  // 创建一个 Cursor 来读取 zip 字节
  let cursor = Cursor::new(zip_bytes);
  let mut archive = ZipArchive::new(cursor)?;
  
  // 存储解压缩的文件内容
  let mut files: HashMap<String, Vec<u8>> = HashMap::new();
  
  // 解压缩所有文件
  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let file_name = file.name().to_string();
    
    // 跳过目录项
    if file_name.ends_with('/') {
      continue;
    }
    
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    
    // 如果是 XML 文件，进行处理
    if file_name.ends_with(".xml") {
      match String::from_utf8(contents.clone()) {
        Ok(xml_content) => {
          let processed_xml = process_xml(&file_name, &xml_content, data)?;
          files.insert(file_name, processed_xml.into_bytes());
        }
        Err(_) => {
          // 如果不是有效的 UTF-8，直接保存原始字节
          files.insert(file_name, contents);
        }
      }
    } else {
      // 非 XML 文件直接保存
      files.insert(file_name, contents);
    }
  }
  
  // 重新压缩文件
  let mut output = Vec::new();
  {
    let cursor = Cursor::new(&mut output);
    let mut zip_writer = ZipWriter::new(cursor);
    
    for (file_name, contents) in files {
      let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .compression_level(Some(6)); // 设置压缩级别
      
      zip_writer.start_file(file_name, options)?;
      zip_writer.write_all(&contents)?;
    }
    
    zip_writer.finish()?;
  }
  
  Ok(output)
}

/// 处理 XML 内容，合并被分割的 Handlebars 模板语法
fn process_xml(
    _file_name: &str,
    content: &str,
    data: &Value,
) -> Result<String, Box<dyn std::error::Error>> {
  // 合并被分割的 Handlebars 语法
  let content = merge_handlebars_in_xml(content)?;
  
  let content = apply_handlebars_template(&content, data)?;
  
  Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::{ZipWriter, write::SimpleFileOptions};

    #[test]
    fn test_render_docx() {
        // 创建一个模拟的 DOCX 文件结构
        let mut zip_data = Vec::new();
        {
            let cursor = std::io::Cursor::new(&mut zip_data);
            let mut zip = ZipWriter::new(cursor);
            let options = SimpleFileOptions::default();
            
            // 添加必需的 DOCX 文件
            
            // 1. [Content_Types].xml
            zip.start_file("[Content_Types].xml", options).unwrap();
            zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#).unwrap();
            
            // 2. _rels/.rels
            zip.start_file("_rels/.rels", options).unwrap();
            zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#).unwrap();
            
            // 3. word/document.xml
            zip.start_file("word/document.xml", options).unwrap();
            zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
<w:p><w:r><w:t>Hello {{name}}</w:t></w:r></w:p>
</w:body>
</w:document>"#).unwrap();
            
            zip.finish().unwrap();
        }
        
        // 创建测试数据
        let test_data = serde_json::json!({
            "name": "World"
        });
        
        // 调用 render 函数
        let result = render_handlebars(zip_data, &test_data);
        
        // 验证结果
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
        
        // 验证输出是有效的 ZIP 文件
        let cursor = std::io::Cursor::new(output);
        let mut archive = ZipArchive::new(cursor).unwrap();
        assert!(archive.len() >= 3); // 至少包含我们添加的3个文件
        
        // 检查 document.xml 是否被正确处理
        let mut document_file = archive.by_name("word/document.xml").unwrap();
        let mut content = String::new();
        document_file.read_to_string(&mut content).unwrap();
        
        // 验证 Handlebars 模板已被渲染
        assert!(content.contains("Hello World"));
        assert!(!content.contains("{{name}}"));
    }
}
