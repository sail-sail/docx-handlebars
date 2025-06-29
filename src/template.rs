use crate::error::{DocxHandlebarsError, Result};
use handlebars::{Handlebars, Helper, Context, RenderContext, Output, HelperResult};
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;

/// Handlebars 模板引擎
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    /// 创建新的模板引擎
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        
        // 注册自定义 helper
        handlebars.register_helper("eq", Box::new(eq_helper));
        handlebars.register_helper("ne", Box::new(ne_helper));
        handlebars.register_helper("gt", Box::new(gt_helper));
        handlebars.register_helper("lt", Box::new(lt_helper));
        handlebars.register_helper("and", Box::new(and_helper));
        handlebars.register_helper("or", Box::new(or_helper));
        handlebars.register_helper("not", Box::new(not_helper));
        handlebars.register_helper("format_number", Box::new(format_number_helper));
        handlebars.register_helper("format_date", Box::new(format_date_helper));
        handlebars.register_helper("upper", Box::new(upper_helper));
        handlebars.register_helper("lower", Box::new(lower_helper));

        Self { handlebars }
    }

    /// 从 XML 内容中提取模板变量
    pub fn extract_variables(&self, xml_content: &str) -> Result<Vec<String>> {
        let text_content = self.extract_text_from_xml(xml_content)?;
        let variables = self.extract_handlebars_variables(&text_content);
        Ok(variables)
    }

    /// 从 XML 中提取纯文本（保留换行信息）
    fn extract_text_from_xml(&self, xml_content: &str) -> Result<String> {
        let mut text_content = String::new();
        let mut reader = Reader::from_str(xml_content);

        let mut buf = Vec::new();
        let mut in_text = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"w:t" {
                        in_text = true;
                    } else if e.name().as_ref() == b"w:p" {
                        // 段落开始，如果不是第一个段落则添加换行
                        if !text_content.is_empty() {
                            text_content.push('\n');
                        }
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
                Ok(Event::Empty(ref e)) => {
                    if e.name().as_ref() == b"w:br" {
                        // 换行标签
                        text_content.push('\n');
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

    /// 提取 Handlebars 变量
    fn extract_handlebars_variables(&self, text: &str) -> Vec<String> {
        let mut variables = HashSet::new();
        
        // 匹配 {{variable}} 和 {{#each items}} 等模式
        let re = Regex::new(r"\{\{[#/]?([^}]+)\}\}").unwrap();
        
        for cap in re.captures_iter(text) {
            if let Some(var_match) = cap.get(1) {
                let var_str = var_match.as_str().trim();
                
                // 过滤掉 helpers 和特殊关键字
                if !var_str.starts_with('#') && !var_str.starts_with('/') {
                    // 提取变量名（去掉 helper 调用）
                    let var_name = var_str.split_whitespace().next().unwrap_or(var_str);
                    if !["if", "unless", "each", "with", "else"].contains(&var_name) {
                        variables.insert(var_name.to_string());
                    }
                }
            }
        }
        
        variables.into_iter().collect()
    }

    /// 渲染 XML 内容
    pub fn render_content(&self, xml_content: &str, data: &Value) -> Result<String> {
        // 检查是否包含表格结构
        let has_table = xml_content.contains("<w:tbl>");
        
        // 检查是否包含复杂的跨段落模板
        let full_text = self.extract_text_from_xml(xml_content)?;
        let has_complex_templates = self.has_complex_cross_paragraph_templates(&full_text);
        
        // 如果既有表格又有复杂模板，使用混合策略
        if has_table && has_complex_templates {
            return self.render_with_table_and_complex_templates(xml_content, data);
        }
        
        // 如果只有表格，使用段落级别渲染以保持结构
        if has_table {
            return self.render_xml_text_per_node_improved(xml_content, data);
        }
        
        // 如果只有复杂模板，使用文档级别渲染
        if has_complex_templates {
            return self.render_document_level_template(xml_content, data);
        }
        
        // 默认情况，使用段落级别渲染
        self.render_xml_text_per_node_improved(xml_content, data)
    }

    /// 检查是否包含复杂的跨段落模板语法
    fn has_complex_cross_paragraph_templates(&self, text: &str) -> bool {
        let has_each = text.contains("{{#each") && text.contains("{{/each}}");
        let has_if = text.contains("{{#if") && text.contains("{{/if}}");
        has_each || has_if
    }

    /// 文档级别的模板渲染
    fn render_document_level_template(&self, xml_content: &str, data: &Value) -> Result<String> {
        // 首先提取完整的文本内容
        let full_text = self.extract_text_from_xml(xml_content)?;
        
        // 渲染完整的模板
        let rendered_text = match self.handlebars.render_template(&full_text, data) {
            Ok(rendered) => rendered,
            Err(e) => {
                eprintln!("模板渲染失败: {}, 回退到段落级别渲染", e);
                return self.render_xml_text_per_node_improved(xml_content, data);
            }
        };
        
        // 将渲染后的文本重新分配到文档结构中
        self.redistribute_rendered_text(xml_content, &rendered_text)
    }

    /// 将渲染后的文本重新分配到文档的段落结构中
    fn redistribute_rendered_text(&self, _xml_content: &str, rendered_text: &str) -> Result<String> {
        use quick_xml::Writer;
        use std::io::Cursor;
        use std::io::Write;
        
        // 为每行文本创建段落
        let text_lines: Vec<&str> = rendered_text.lines().collect();
        let mut result: Vec<u8> = Vec::new();
        let mut writer = Writer::new(Cursor::new(&mut result));
        
        // 写入文档开始部分
        let doc_start = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>"#;
        writer.get_mut().write_all(doc_start.as_bytes())?;
        
        // 为每行文本创建段落
        for line in text_lines {
            if line.trim().is_empty() {
                // 空行创建空段落
                let empty_para = r#"<w:p>
  <w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="宋体" w:hAnsi="宋体"/>
    </w:rPr>
  </w:pPr>
</w:p>"#;
                writer.get_mut().write_all(empty_para.as_bytes())?;
            } else {
                // 有内容的行
                let para_content = format!(r#"<w:p>
  <w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="宋体" w:hAnsi="宋体"/>
    </w:rPr>
  </w:pPr>
  <w:r>
    <w:rPr>
      <w:rFonts w:ascii="宋体" w:hAnsi="宋体"/>
    </w:rPr>
    <w:t>{}</w:t>
  </w:r>
</w:p>"#, line);
                writer.get_mut().write_all(para_content.as_bytes())?;
            }
        }
        
        // 写入文档结束部分
        let doc_end = r#"</w:body>
</w:document>"#;
        writer.get_mut().write_all(doc_end.as_bytes())?;
        
        String::from_utf8(result).map_err(|e| DocxHandlebarsError::Custom(format!("UTF-8 转换错误: {}", e)))
    }
    
    /// 改进的逐节点渲染，更好地处理跨节点的 Handlebars 语法，包括表格支持
    fn render_xml_text_per_node_improved(&self, xml_content: &str, data: &Value) -> Result<String> {
        use quick_xml::Writer;
        use std::io::Cursor;
        
        let mut result: Vec<u8> = Vec::new();
        let mut reader = Reader::from_str(xml_content);
        let mut writer = Writer::new(Cursor::new(&mut result));
        
        let mut buf = Vec::new();
        let mut in_paragraph = false;
        let mut paragraph_texts = Vec::new();
        let mut current_paragraph_events: Vec<Event<'static>> = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"w:p" => {
                            in_paragraph = true;
                            paragraph_texts.clear();
                            current_paragraph_events.clear();
                            current_paragraph_events.push(Event::Start(e.clone().into_owned()));
                        }
                        b"w:tc" => {
                            // 表格单元格开始，直接传递
                            writer.write_event(Event::Start(e.clone()))?;
                        }
                        b"w:tbl" | b"w:tr" | b"w:tblPr" | b"w:tcPr" | b"w:tblGrid" | b"w:gridCol" => {
                            // 表格相关元素，直接传递
                            writer.write_event(Event::Start(e.clone()))?;
                        }
                        _ => {
                            if in_paragraph {
                                current_paragraph_events.push(Event::Start(e.clone().into_owned()));
                            } else {
                                writer.write_event(Event::Start(e.clone()))?;
                            }
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_paragraph {
                        // 检查是否在 w:t 元素内
                        let is_in_text_element = current_paragraph_events.iter().rev()
                            .any(|event| matches!(event, Event::Start(start_event) if start_event.name().as_ref() == b"w:t"));
                        
                        if is_in_text_element {
                            paragraph_texts.push(e.unescape()?.to_string());
                        }
                        current_paragraph_events.push(Event::Text(e.into_owned()));
                    } else {
                        writer.write_event(Event::Text(e))?;
                    }
                }
                Ok(Event::End(ref e)) => {
                    match e.name().as_ref() {
                        b"w:p" => {
                            // 段落结束，处理整个段落的文本
                            let paragraph_text = paragraph_texts.join("");
                            
                            if paragraph_text.contains("{{") && paragraph_text.contains("}}") {
                                // 尝试渲染段落文本
                                let rendered_paragraph = match self.handlebars.render_template(&paragraph_text, data) {
                                    Ok(rendered) => rendered,
                                    Err(_) => paragraph_text.clone(),
                                };
                                
                                // 检查是否包含复杂的模板语法
                                let has_complex_syntax = paragraph_text.contains("{{#each") 
                                    || paragraph_text.contains("{{#if") 
                                    || paragraph_text.contains("{{/each}}")
                                    || paragraph_text.contains("{{/if}}");
                                
                                if has_complex_syntax {
                                    // 复杂模板，使用简化处理 - 放在第一个文本节点
                                    let mut text_written = false;
                                    let mut in_text_element = false;
                                    for event in &current_paragraph_events {
                                        match event {
                                            Event::Start(e) if e.name().as_ref() == b"w:t" => {
                                                in_text_element = true;
                                                writer.write_event(Event::Start(e.clone()))?;
                                            }
                                            Event::Text(_) if in_text_element => {
                                                if !text_written {
                                                    let text_event = quick_xml::events::BytesText::new(&rendered_paragraph);
                                                    writer.write_event(Event::Text(text_event))?;
                                                    text_written = true;
                                                }
                                            }
                                            Event::End(e) if e.name().as_ref() == b"w:t" => {
                                                in_text_element = false;
                                                writer.write_event(Event::End(e.clone()))?;
                                            }
                                            _ => {
                                                writer.write_event(event.clone())?;
                                            }
                                        }
                                    }
                                } else {
                                    // 简单模板，应用渲染结果但保持格式
                                    let mut text_written = false;
                                    let mut in_text_element = false;
                                    for event in &current_paragraph_events {
                                        match event {
                                            Event::Start(e) if e.name().as_ref() == b"w:t" => {
                                                in_text_element = true;
                                                writer.write_event(Event::Start(e.clone()))?;
                                            }
                                            Event::Text(_) if in_text_element => {
                                                if !text_written {
                                                    // 在第一个文本节点写入渲染后的内容
                                                    let text_event = quick_xml::events::BytesText::new(&rendered_paragraph);
                                                    writer.write_event(Event::Text(text_event))?;
                                                    text_written = true;
                                                }
                                                // 后续文本节点跳过
                                            }
                                            Event::End(e) if e.name().as_ref() == b"w:t" => {
                                                in_text_element = false;
                                                writer.write_event(Event::End(e.clone()))?;
                                            }
                                            _ => {
                                                writer.write_event(event.clone())?;
                                            }
                                        }
                                    }
                                }
                            } else {
                                // 没有模板语法，直接写入原始段落
                                for event in &current_paragraph_events {
                                    writer.write_event(event.clone())?;
                                }
                            }
                            
                            writer.write_event(Event::End(e.clone()))?;
                            in_paragraph = false;
                        }
                        b"w:tc" => {
                            // 表格单元格结束，直接传递
                            writer.write_event(Event::End(e.clone()))?;
                        }
                        b"w:tbl" | b"w:tr" | b"w:tblPr" | b"w:tcPr" | b"w:tblGrid" | b"w:gridCol" => {
                            // 表格相关元素，直接传递
                            writer.write_event(Event::End(e.clone()))?;
                        }
                        _ => {
                            if in_paragraph {
                                current_paragraph_events.push(Event::End(e.clone().into_owned()));
                            } else {
                                writer.write_event(Event::End(e.clone()))?;
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Ok(event) => {
                    if in_paragraph {
                        current_paragraph_events.push(event.into_owned());
                    } else {
                        writer.write_event(event)?;
                    }
                }
                Err(e) => return Err(DocxHandlebarsError::Xml(e)),
            }
            buf.clear();
        }
        
        String::from_utf8(result).map_err(|e| DocxHandlebarsError::Custom(format!("UTF-8 转换错误: {}", e)))
    }
    
    /// 混合渲染：既保留表格结构又处理复杂模板
    fn render_with_table_and_complex_templates(&self, xml_content: &str, data: &Value) -> Result<String> {
        // 直接调用结构保持渲染
        self.redistribute_text_preserving_structure(xml_content, "", data)
    }

    /// 重新分配文本但保持XML结构（包括表格）
    fn redistribute_text_preserving_structure(&self, xml_content: &str, rendered_text: &str, data: &Value) -> Result<String> {
        // 找到表格的位置，将内容分为表格前、表格和表格后三部分
        let table_start = xml_content.find("<w:tbl>");
        let table_end = xml_content.find("</w:tbl>");
        
        if let (Some(start), Some(end)) = (table_start, table_end) {
            let table_end_pos = end + "</w:tbl>".len();
            
            // 找到 body 结束的位置来正确分割
            let body_end = xml_content.find("</w:body>").unwrap_or(xml_content.len());
            
            let before_table = &xml_content[..start];
            let table_content = &xml_content[start..table_end_pos];
            let after_table = &xml_content[table_end_pos..body_end]; // 只到body结束前
            
            // 分别提取表格前后的原始文本内容（用于单独渲染）
            let before_table_wrapped = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
{}
</w:body>
</w:document>"#,
                before_table
            );
            
            let before_table_text = self.extract_text_from_xml(&before_table_wrapped)?;
            
            // 为表格后部分创建完整的XML包装
            let after_table_wrapped = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
{}
</w:body>
</w:document>"#,
                after_table
            );
            
            let after_table_text = self.extract_text_from_xml(&after_table_wrapped)?;
            
            // 分别渲染表格前后的内容
            let before_rendered = if !before_table_text.trim().is_empty() {
                match self.handlebars.render_template(&before_table_text, data) {
                    Ok(text) => text,
                    Err(_) => before_table_text
                }
            } else {
                String::new()
            };
            
            let after_rendered = if !after_table_text.trim().is_empty() {
                match self.handlebars.render_template(&after_table_text, data) {
                    Ok(text) => text,
                    Err(_) => after_table_text
                }
            } else {
                String::new()
            };
            
            // 重新构建文档
            let mut result = String::new();
            
            // 添加XML声明和文档开始（保持原有的namespace声明）
            if let Some(body_start) = xml_content.find("<w:body>") {
                result.push_str(&xml_content[..body_start + "<w:body>".len()]);
            } else {
                // 如果找不到body标签，使用默认开头
                result.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>"#);
            }
            
            // 添加表格前的段落
            for line in before_rendered.lines() {
                if !line.trim().is_empty() {
                    result.push_str(&format!(r#"<w:p>
  <w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="宋体" w:hAnsi="宋体"/>
    </w:rPr>
  </w:pPr>
  <w:r>
    <w:rPr>
      <w:rFonts w:ascii="宋体" w:hAnsi="宋体"/>
    </w:rPr>
    <w:t>{}</w:t>
  </w:r>
</w:p>"#, line));
                } else {
                    result.push_str(r#"<w:p>
  <w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="宋体" w:hAnsi="宋体"/>
    </w:rPr>
  </w:pPr>
</w:p>"#);
                }
            }
            
            // 添加表格（保持原样）
            result.push_str(table_content);
            
            // 添加表格后的段落
            for line in after_rendered.lines() {
                if !line.trim().is_empty() {
                    result.push_str(&format!(r#"<w:p>
  <w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="宋体" w:hAnsi="宋体"/>
    </w:rPr>
  </w:pPr>
  <w:r>
    <w:rPr>
      <w:rFonts w:ascii="宋体" w:hAnsi="宋体"/>
    </w:rPr>
    <w:t>{}</w:t>
  </w:r>
</w:p>"#, line));
                } else {
                    result.push_str(r#"<w:p>
  <w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="宋体" w:hAnsi="宋体"/>
    </w:rPr>
  </w:pPr>
</w:p>"#);
                }
            }
            
            // 添加文档结束（保持原有的sectPr等结构）
            if let Some(body_end) = xml_content.find("</w:body>") {
                result.push_str(&xml_content[body_end..]);
            } else {
                result.push_str("</w:body></w:document>");
            }
            
            Ok(result)
        } else {
            // 如果没有表格，回退到原有逻辑
            self.redistribute_rendered_text(xml_content, rendered_text)
        }
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

// 自定义 Handlebars helpers

fn eq_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param1 = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    let param2 = h.param(1).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(&(param1 == param2).to_string())?;
    Ok(())
}

fn ne_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param1 = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    let param2 = h.param(1).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(&(param1 != param2).to_string())?;
    Ok(())
}

fn gt_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param1 = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    let param2 = h.param(1).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    out.write(&(param1 > param2).to_string())?;
    Ok(())
}

fn lt_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param1 = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    let param2 = h.param(1).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    out.write(&(param1 < param2).to_string())?;
    Ok(())
}

fn and_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param1 = h.param(0).and_then(|v| v.value().as_bool()).unwrap_or(false);
    let param2 = h.param(1).and_then(|v| v.value().as_bool()).unwrap_or(false);
    out.write(&(param1 && param2).to_string())?;
    Ok(())
}

fn or_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param1 = h.param(0).and_then(|v| v.value().as_bool()).unwrap_or(false);
    let param2 = h.param(1).and_then(|v| v.value().as_bool()).unwrap_or(false);
    out.write(&(param1 || param2).to_string())?;
    Ok(())
}

fn not_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_bool()).unwrap_or(false);
    out.write(&(!param).to_string())?;
    Ok(())
}

fn format_number_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let number = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    let formatted = format!("{:.2}", number);
    out.write(&formatted)?;
    Ok(())
}

fn format_date_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let date_str = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    // 这里可以实现更复杂的日期格式化逻辑
    out.write(date_str)?;
    Ok(())
}

fn upper_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let text = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(&text.to_uppercase())?;
    Ok(())
}

fn lower_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let text = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(&text.to_lowercase())?;
    Ok(())
}
