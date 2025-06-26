use docx_handlebars::{DocxHandlebars, Result};
use serde_json::json;
use std::collections::HashMap;

fn main() -> Result<()> {
    // 创建处理器实例
    let mut processor = DocxHandlebars::new();
    
    // 模拟一个简单的 DOCX 文件内容（实际使用中应该从文件读取）
    println!("这是一个基本的使用示例");
    println!("在实际使用中，您需要:");
    println!("1. 准备一个包含 Handlebars 模板的 DOCX 文件");
    println!("2. 读取文件内容到字节数组");
    println!("3. 使用 load_template_bytes 方法加载模板");
    println!("4. 准备数据并调用 render_with_data 方法");
    
    // 示例数据
    let data = json!({
        "name": "张三",
        "company": "ABC科技有限公司",
        "position": "软件工程师",
        "salary": 15000,
        "start_date": "2024-01-15",
        "projects": [
            {
                "name": "项目A",
                "description": "企业管理系统",
                "status": "已完成"
            },
            {
                "name": "项目B", 
                "description": "移动应用开发",
                "status": "进行中"
            }
        ],
        "skills": ["Rust", "JavaScript", "Python", "React"],
        "has_bonus": true,
        "bonus_amount": 5000
    });
    
    println!("\n示例数据:");
    println!("{}", serde_json::to_string_pretty(&data)?);
    
    // 在真实环境中的使用方法:
    println!("\n真实使用代码示例:");
    println!(r#"
// 读取模板文件
let template_bytes = std::fs::read("template.docx")?;

// 加载模板
processor.load_template_bytes(&template_bytes)?;

// 渲染模板
let result = processor.render_with_data(&data)?;

// 保存结果
std::fs::write("output.docx", result)?;
"#);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = DocxHandlebars::new();
        // 基本的创建测试
        assert!(true); // 如果能创建就通过
    }

    #[test]
    fn test_data_serialization() {
        let data = json!({
            "name": "测试",
            "value": 123
        });
        
        let serialized = serde_json::to_string(&data).unwrap();
        assert!(serialized.contains("测试"));
        assert!(serialized.contains("123"));
    }
}
