use serde_json::Value;
use std::{io::{Cursor, Read, Write}, sync::{Arc, Mutex}};
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};
use std::collections::HashMap;
use crate::{utils::{get_image_dimensions, merge_handlebars_in_xml, register_basic_helpers, remove_table_row_simple, validate_docx_format}, DocxError};

use handlebars::{Handlebars, RenderErrorReason, handlebars_helper};
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};

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
  let files: Arc<Mutex<HashMap<String, Vec<u8>>>> = Arc::new(Mutex::new(HashMap::new()));
  
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
    files.lock().map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to lock files: {}", e))))?.insert(file_name, contents);
  }
  
  // 处理 document.xml 文件
  {
    let file_name = "word/document.xml";
    let contents = files.lock().map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to lock files: {}", e))))?.remove(file_name);
    if let Some(contents) = contents {
      let xml_content = String::from_utf8(contents.clone())?;
      
      let xml_content = merge_handlebars_in_xml(xml_content)?;
  
      let mut handlebars = Handlebars::new();
      
      handlebars.set_strict_mode(false); // 允许未定义的变量
      
      register_basic_helpers(&mut handlebars)?;
      
      let files_ref = Arc::clone(&files);
      // 注册 img helper
      handlebars.register_helper("img", Box::new(
        move |
          h: &handlebars::Helper,
          _r: &Handlebars,
          _ctx: &handlebars::Context,
          _rc: &mut handlebars::RenderContext,
          out: &mut dyn handlebars::Output
        | -> Result<(), handlebars::RenderError> {
          let src = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
          if src.is_empty() {
            out.write("")?;
            return Ok(());
          }
          let width_param = h.param(1).and_then(|v| v.value().as_u64());
          let height_param = h.param(2).and_then(|v| v.value().as_u64());
          
          let rid = Uuid::new_v4().to_string().replace("-", "");
          let rid = format!("a{rid}");
          let mut files_mut = files_ref.lock().map_err(|e| {
            RenderErrorReason::Other(e.to_string())
          })?;
          
          {
            let file_name = "word/_rels/document.xml.rels";
            if let Some(contents) = files_mut.remove(file_name) {
              let rels_content = String::from_utf8(contents)?;
              let new_rels_content = rels_content.replace(
                "</Relationships>",
                &format!(
                  "<Relationship Id=\"{rid}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/image\" Target=\"media/{rid}.png\"></Relationship></Relationships>",
                ),
              );
              files_mut.insert(file_name.to_string(), new_rels_content.into_bytes());
            } else {
              // 如果没有找到关系文件，则创建一个新的
              let new_rels_content = format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship Id=\"{rid}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/image\" Target=\"media/{rid}.png\"></Relationship></Relationships>",
              );
              files_mut.insert("word/_rels/document.xml.rels".to_string(), new_rels_content.into_bytes());
            }
          }
          
          {
            let file_name = format!("word/media/{rid}.png");
            // image_data 写入文件系统用于调试
            // std::fs::write(format!("D:/{rid}.base64"), &src)?;
            // src 是base64 编码的图片数据
            let image_data = general_purpose::STANDARD.decode(src).map_err(|e| {
              RenderErrorReason::Other(format!("Failed to decode base64 image: {}", e))
            })?;
            
            // 获取图片的宽高
            let (orig_w, orig_h) = get_image_dimensions(&image_data).ok_or_else(|| {
              RenderErrorReason::Other(format!("Failed to get image dimensions"))
            })?;
            
            // 计算目标宽高
            let (target_w, target_h) = match (width_param, height_param) {
              (Some(w), Some(h)) => (w as u32, h as u32),
              (Some(w), None) => {
                // 只传 width，等比缩放 height
                let h = (orig_h as f64 * w as f64 / orig_w as f64).round() as u32;
                (w as u32, h)
              }
              (None, Some(h)) => {
                // 只传 height，等比缩放 width
                let w = (orig_w as f64 * h as f64 / orig_h as f64).round() as u32;
                (w, h as u32)
              }
              (None, None) => (orig_w, orig_h),
            };
            
            files_mut.insert(file_name, image_data);
            
            let mut str = String::new();
            
            // 根据图片宽高计算 cx 和 cy
            let cx = target_w * 9525; // 1px = 9525 EMU
            let cy = target_h * 9525; // 1px = 9525 EMU
            
            str.push_str("<w:drawing>");
              str.push_str("<wp:inline distT=\"0\" distB=\"0\" distL=\"0\" distR=\"0\" wp14:anchorId=\"4A52EBB1\" wp14:editId=\"5A58C96F\">");
                // str.push_str("<wp:extent cx=\"2216867\" cy=\"742950\" />");
                str.push_str(&format!("<wp:extent cx=\"{cx}\" cy=\"{cy}\" />"));
                str.push_str("<wp:effectExtent l=\"0\" t=\"0\" r=\"0\" b=\"0\" />");
                str.push_str(&format!("<wp:docPr id=\"873087433\" name=\"{rid}\" />")); // 图片 1
                str.push_str("<wp:cNvGraphicFramePr>");
                  str.push_str("<a:graphicFrameLocks xmlns:a=\"http://schemas.openxmlformats.org/drawingml/2006/main\" noChangeAspect=\"1\" />");
                str.push_str("</wp:cNvGraphicFramePr>");
                str.push_str("<a:graphic xmlns:a=\"http://schemas.openxmlformats.org/drawingml/2006/main\">");
                  str.push_str("<a:graphicData uri=\"http://schemas.openxmlformats.org/drawingml/2006/picture\">");
                    str.push_str("<pic:pic xmlns:pic=\"http://schemas.openxmlformats.org/drawingml/2006/picture\">");
                      str.push_str("<pic:nvPicPr>");
                        str.push_str(&format!("<pic:cNvPr id=\"873087433\" name=\"{rid}\" />")); // 图片 873087433
                        str.push_str("<pic:cNvPicPr />");
                      str.push_str("</pic:nvPicPr>");
                      str.push_str("<pic:blipFill>");
                        str.push_str(&format!("<a:blip r:embed=\"{rid}\">")); // rId4
                          str.push_str("<a:extLst>");
                            str.push_str("<a:ext uri=\"{28A0092B-C50C-407E-A947-70E740481C1C}\">");
                              str.push_str("<a14:useLocalDpi xmlns:a14=\"http://schemas.microsoft.com/office/drawing/2010/main\" val=\"0\" />");
                            str.push_str("</a:ext>");
                          str.push_str("</a:extLst>");
                        str.push_str("</a:blip>");
                        str.push_str("<a:stretch>");
                          str.push_str("<a:fillRect />");
                        str.push_str("</a:stretch>");
                      str.push_str("</pic:blipFill>");
                      str.push_str("<pic:spPr>");
                        str.push_str("<a:xfrm>");
                          str.push_str("<a:off x=\"0\" y=\"0\" />");
                          // str.push_str("<a:ext cx=\"2220115\" cy=\"744039\" />");
                          str.push_str(&format!("<a:ext cx=\"{cx}\" cy=\"{cy}\" />"));
                        str.push_str("</a:xfrm>");
                        str.push_str("<a:prstGeom prst=\"rect\">");
                          str.push_str("<a:avLst />");
                        str.push_str("</a:prstGeom>");
                      str.push_str("</pic:spPr>");
                    str.push_str("</pic:pic>");
                  str.push_str("</a:graphicData>");
                str.push_str("</a:graphic>");
              str.push_str("</wp:inline>");
            str.push_str("</w:drawing>");
          
            out.write(&str)?;
          }
          
          // [Content_Types].xml
          {
            let file_name = "[Content_Types].xml";
            if let Some(contents) = files_mut.remove(file_name) {
              let content_types_content = String::from_utf8(contents)?;
              
              if !content_types_content.contains(" Extension=\"png\" ") {
                let new_content_types_content = content_types_content.replace(
                  "</Types>",
                  "<Default Extension=\"png\" ContentType=\"image/png\" /></Types>",
                );
                files_mut.insert(file_name.to_string(), new_content_types_content.into_bytes());
              } else {
                // 如果已经存在 png 的默认类型，则不重复添加
                files_mut.insert(file_name.to_string(), content_types_content.into_bytes());
              }
              
            } else {
              // 如果没有找到 [Content_Types].xml 文件，则创建一个新的
              let new_content_types_content = format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default Extension=\"png\" ContentType=\"image/png\" /></Types>",
              );
              files_mut.insert(file_name.to_string(), new_content_types_content.into_bytes());
            }
          }
          
          Ok(())
        },
      ));
      
      // 标记删除表格的一行
      handlebars_helper!(removeTableRow: | | {
        "d53e6de6-fb82-4ca8-95aa-2bc56b6d5791"
      });
      handlebars.register_helper("removeTableRow", Box::new(removeTableRow));
      
      // 渲染模板
      let xml_content = handlebars.render_template(&xml_content, data)
        .map_err(|e| {
          let reason: &RenderErrorReason = e.reason();
          DocxError::TemplateRenderError(reason.to_string())
        })?;
      
      let xml_content = fix_drawing_with_placeholders(xml_content)?;
      
      // std::fs::write("./document.xml", &xml_content)?;
      
      files.lock().map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to lock files: {}", e))))?.insert(file_name.to_string(), xml_content.into_bytes());
    }
  }
  
  // Extract files from Arc<Mutex<_>>
  let files = Arc::try_unwrap(files).map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to unwrap Arc")))?.into_inner().map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to get inner value: {:?}", e))))?;
  
  // 重新压缩文件
  let mut output = Vec::new();
  {
    let cursor = Cursor::new(&mut output);
    let mut zip_writer = ZipWriter::new(cursor);
    
    for entry in files {
      let (file_name, contents): (String, Vec<u8>) = entry;
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

fn fix_drawing_with_placeholders(xml_content: String) -> Result<String, Box<dyn std::error::Error>> {
  let mut fixed_content = xml_content;
  
  fixed_content = fixed_content.replace("<w:t><w:drawing>", "<w:drawing>");
  fixed_content = fixed_content.replace("</w:drawing></w:t>", "</w:drawing>");
  
  fixed_content = remove_table_row_simple(
    &fixed_content,
    "d53e6de6-fb82-4ca8-95aa-2bc56b6d5791",
  )?;
  
  Ok(fixed_content)
}
