//! 错误类型定义

use thiserror::Error;

/// DOCX 处理错误类型
#[derive(Error, Debug)]
pub enum DocxError {
    #[error("无效的 DOCX 文件格式：文件不是有效的 ZIP 文件")]
    InvalidZipFormat,
    #[error("文件大小不足：文件太小，不是有效的 ZIP 文件（需要至少 22 字节）")]
    FileTooSmall,
    #[error("无效的 ZIP 签名：文件开头不是有效的 ZIP 文件签名")]
    InvalidZipSignature,
    #[error("缺少必需的 DOCX 文件：{0}")]
    MissingRequiredFile(String),
    #[error("ZIP 文件读取错误：{0}")]
    ZipReadError(String),
    #[error("IO 错误：{0}")]
    IoError(#[from] std::io::Error),
    #[error("模板处理错误：{0}")]
    TemplateError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docx_error_display() {
        // 测试错误信息的显示
        let file_too_small = DocxError::FileTooSmall;
        assert_eq!(
            file_too_small.to_string(),
            "文件大小不足：文件太小，不是有效的 ZIP 文件（需要至少 22 字节）"
        );
        
        let invalid_signature = DocxError::InvalidZipSignature;
        assert_eq!(
            invalid_signature.to_string(),
            "无效的 ZIP 签名：文件开头不是有效的 ZIP 文件签名"
        );
        
        let missing_file = DocxError::MissingRequiredFile("word/document.xml".to_string());
        assert_eq!(
            missing_file.to_string(),
            "缺少必需的 DOCX 文件：word/document.xml"
        );
        
        let template_error = DocxError::TemplateError("测试错误".to_string());
        assert_eq!(
            template_error.to_string(),
            "模板处理错误：测试错误"
        );
    }
}
