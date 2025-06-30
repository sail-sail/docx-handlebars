use serde_json::Value;
use std::io::{Cursor, Read, Write};
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};
use std::collections::HashMap;
use crate::utils::merge_handlebars_in_xml;

pub fn render_handlebars(
  zip_bytes: Vec<u8>,
  data: &Value,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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
    xml_content: &str,
    _data: &Value,
) -> Result<String, Box<dyn std::error::Error>> {
  // 合并被分割的 Handlebars 语法
  let merged_content = merge_handlebars_in_xml(xml_content)?;
  
  // TODO: 在这里添加 Handlebars 模板引擎处理
  // let processed_content = apply_handlebars_template(&merged_content, data)?;
  
  Ok(merged_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::{ZipWriter, write::SimpleFileOptions};

    #[test]
    fn test_render_docx() {
        // 创建一个简单的测试 ZIP 文件
        let mut zip_data = Vec::new();
        {
            let cursor = std::io::Cursor::new(&mut zip_data);
            let mut zip = ZipWriter::new(cursor);
            
            // 添加一个测试 XML 文件
            let options = SimpleFileOptions::default();
            zip.start_file("test.xml", options).unwrap();
            zip.write_all(b"<root>Hello World</root>").unwrap();
            
            // 添加一个非 XML 文件
            zip.start_file("test.txt", options).unwrap();
            zip.write_all(b"Hello from text file").unwrap();
            
            zip.finish().unwrap();
        }
        
        // 创建测试数据
        let test_data = serde_json::json!({
            "name": "Test",
            "value": 123
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
        assert_eq!(archive.len(), 2);
        
        // 检查文件是否存在
        let file_names: Vec<String> = (0..archive.len())
            .map(|i| archive.by_index(i).unwrap().name().to_string())
            .collect();
        
        assert!(file_names.contains(&"test.xml".to_string()));
        assert!(file_names.contains(&"test.txt".to_string()));
    }
}
