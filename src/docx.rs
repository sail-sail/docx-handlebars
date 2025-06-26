use crate::error::{DocxHandlebarsError, Result};
use quick_xml::events::{Event, BytesText};
use quick_xml::{Reader, Writer};
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use zip::{ZipArchive, ZipWriter, write::FileOptions};

/// DOCX 文档处理器
pub struct DocxProcessor {
    /// 原始 ZIP 文件内容
    zip_content: Vec<u8>,
    /// 文档主要内容
    document_content: String,
    /// 其他文件内容（如样式、关系等）
    other_files: HashMap<String, Vec<u8>>,
}

impl DocxProcessor {
    /// 创建新的 DOCX 处理器
    pub fn new() -> Self {
        Self {
            zip_content: Vec::new(),
            document_content: String::new(),
            other_files: HashMap::new(),
        }
    }

    /// 从字节数组加载 DOCX 文件
    pub fn load_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.zip_content = bytes.to_vec();
        self.extract_contents()?;
        Ok(())
    }

    /// 提取 DOCX 文件内容
    fn extract_contents(&mut self) -> Result<()> {
        let cursor = Cursor::new(&self.zip_content);
        let mut archive = ZipArchive::new(cursor)?;

        // 清空之前的内容
        self.other_files.clear();
        self.document_content.clear();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            let file_name = file.name().to_string();
            
            if file_name == "word/document.xml" {
                // 提取主文档内容
                self.document_content = String::from_utf8_lossy(&contents).to_string();
            } else {
                // 存储其他文件
                self.other_files.insert(file_name, contents);
            }
        }

        if self.document_content.is_empty() {
            return Err(DocxHandlebarsError::document_format(
                "无法找到 word/document.xml 文件"
            ));
        }

        Ok(())
    }

    /// 获取文档内容
    pub fn get_content(&self) -> &str {
        &self.document_content
    }

    /// 使用新内容创建 DOCX 文件
    pub fn create_docx_with_content(&self, new_content: &str) -> Result<Vec<u8>> {
        let mut result = Vec::new();
        {
            let cursor = Cursor::new(&mut result);
            let mut zip = ZipWriter::new(cursor);
            let options = FileOptions::<()>::default()
                .compression_method(zip::CompressionMethod::Deflated);

            // 写入新的文档内容
            zip.start_file("word/document.xml", options)?;
            zip.write_all(new_content.as_bytes())?;

            // 写入其他文件
            for (file_name, contents) in &self.other_files {
                zip.start_file(file_name, options)?;
                zip.write_all(contents)?;
            }

            zip.finish()?;
        }

        Ok(result)
    }

    /// 提取纯文本内容（用于模板变量提取）
    pub fn extract_text_content(&self) -> Result<String> {
        let mut text_content = String::new();
        let mut reader = Reader::from_str(&self.document_content);

        let mut buf = Vec::new();
        let mut in_text = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"w:t" {
                        in_text = true;
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_text {
                        text_content.push_str(e.unescape()?.as_ref());
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"w:t" {
                        in_text = false;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(DocxHandlebarsError::Xml(e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(text_content)
    }

    /// 处理 XML 内容中的 Handlebars 模板
    pub fn process_template_in_xml(&self, _template_content: &str) -> Result<String> {
        let mut result = Vec::new();
        let mut reader = Reader::from_str(&self.document_content);
        let mut writer = Writer::new(Cursor::new(&mut result));

        let mut buf = Vec::new();
        let mut in_text = false;
        let mut current_text = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"w:t" {
                        in_text = true;
                        writer.write_event(Event::Start(e.clone()))?;
                    } else {
                        writer.write_event(Event::Start(e.clone()))?;
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_text {
                        current_text = e.unescape()?.to_string();
                        // 这里会在模板引擎中处理
                        let processed_text = current_text.clone();
                        writer.write_event(Event::Text(BytesText::new(&processed_text)))?;
                    } else {
                        writer.write_event(Event::Text(e))?;
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"w:t" {
                        in_text = false;
                        current_text.clear();
                    }
                    writer.write_event(Event::End(e.clone()))?;
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(DocxHandlebarsError::Xml(e)),
                _ => {
                    let event = reader.read_event_into(&mut buf)?;
                    writer.write_event(event)?;
                }
            }
            buf.clear();
        }

        let result_xml = String::from_utf8_lossy(&result).to_string();
        Ok(result_xml)
    }
}

impl Default for DocxProcessor {
    fn default() -> Self {
        Self::new()
    }
}
