//! 错误类型定义

use thiserror::Error;

/// DOCX 处理错误类型
#[derive(Error, Debug)]
pub enum DocxError {
    #[error("无效的 DOCX 文件格式")]
    InvalidZipFormat,
    #[error("{0}")]
    TemplateRenderError(String),
}
