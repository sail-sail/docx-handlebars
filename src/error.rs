use thiserror::Error;

/// docx-handlebars 库的错误类型
#[derive(Error, Debug)]
pub enum DocxHandlebarsError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP 格式错误: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("XML 解析错误: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("Handlebars 模板错误: {0}")]
    Template(#[from] handlebars::TemplateError),

    #[error("Handlebars 渲染错误: {0}")]
    Render(#[from] handlebars::RenderError),

    #[error("JSON 序列化/反序列化错误: {0}")]
    Json(#[from] serde_json::Error),

    #[error("UTF-8 编码错误: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("文档格式错误: {0}")]
    DocumentFormat(String),

    #[error("模板未加载")]
    TemplateNotLoaded,

    #[error("不支持的功能: {0}")]
    UnsupportedFeature(String),

    #[error("自定义错误: {0}")]
    Custom(String),
}

/// 便利类型别名
pub type Result<T> = std::result::Result<T, DocxHandlebarsError>;

impl DocxHandlebarsError {
    /// 创建自定义错误
    pub fn custom<S: Into<String>>(message: S) -> Self {
        Self::Custom(message.into())
    }

    /// 创建文档格式错误
    pub fn document_format<S: Into<String>>(message: S) -> Self {
        Self::DocumentFormat(message.into())
    }

    /// 创建不支持的功能错误
    pub fn unsupported_feature<S: Into<String>>(feature: S) -> Self {
        Self::UnsupportedFeature(feature.into())
    }
}
