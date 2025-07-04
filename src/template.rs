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
          let processed_xml = process_xml(&file_name, xml_content, data)?;
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
    content: String,
    data: &Value,
) -> Result<String, Box<dyn std::error::Error>> {
  // 合并被分割的 Handlebars 语法
  let content = merge_handlebars_in_xml(content)?;
  
  let content = apply_handlebars_template(&content, data)?;
  
  Ok(content)
}
