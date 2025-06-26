use docx_handlebars::{DocxHandlebars, DocxHandlebarsError};
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_processor() {
        let _processor = DocxHandlebars::new();
        // 基本创建测试
        assert!(true);
    }

    #[test]
    fn test_json_serialization() {
        let data = json!({
            "name": "测试用户",
            "age": 25,
            "skills": ["Rust", "JavaScript"],
            "active": true
        });

        let serialized = serde_json::to_string(&data).unwrap();
        assert!(serialized.contains("测试用户"));
        assert!(serialized.contains("25"));
        assert!(serialized.contains("Rust"));
        assert!(serialized.contains("true"));
    }

    #[test] 
    fn test_default_implementations() {
        // 测试 Default trait 实现
        let _processor1 = DocxHandlebars::new();
        let _processor2 = DocxHandlebars::default();
        
        // 两种创建方式应该都能工作
        assert!(true);
    }

    #[test]
    fn test_error_types() {
        // 测试自定义错误
        let custom_error = DocxHandlebarsError::custom("测试错误");
        assert!(custom_error.to_string().contains("测试错误"));

        // 测试文档格式错误
        let format_error = DocxHandlebarsError::document_format("无效格式");
        assert!(format_error.to_string().contains("无效格式"));

        // 测试不支持的功能错误
        let unsupported_error = DocxHandlebarsError::unsupported_feature("某个功能");
        assert!(unsupported_error.to_string().contains("某个功能"));
    }

    #[test]
    fn test_complex_data_structure() {
        let complex_data = json!({
            "company": {
                "name": "测试公司",
                "employees": [
                    {
                        "name": "张三",
                        "position": "开发者",
                        "skills": ["Rust", "Python"]
                    },
                    {
                        "name": "李四",
                        "position": "设计师",
                        "skills": ["UI", "UX"]
                    }
                ]
            },
            "projects": [
                {
                    "name": "项目A",
                    "status": "进行中",
                    "members": ["张三", "李四"]
                }
            ],
            "metadata": {
                "created": "2024-01-01",
                "version": "1.0"
            }
        });

        let serialized = serde_json::to_string_pretty(&complex_data).unwrap();
        assert!(serialized.contains("测试公司"));
        assert!(serialized.contains("张三"));
        assert!(serialized.contains("项目A"));
    }

    #[test]
    fn test_handlebars_template_syntax() {
        use handlebars::Handlebars;
        
        let handlebars = Handlebars::new();
        
        // 测试基本变量替换
        let template1 = "Hello {{name}}!";
        let data1 = json!({"name": "World"});
        let result1 = handlebars.render_template(template1, &data1).unwrap();
        assert_eq!(result1, "Hello World!");

        // 测试条件语句
        let template2 = "{{#if active}}用户活跃{{else}}用户不活跃{{/if}}";
        let data2 = json!({"active": true});
        let result2 = handlebars.render_template(template2, &data2).unwrap();
        assert_eq!(result2, "用户活跃");

        // 测试循环
        let template3 = "{{#each items}}{{name}} {{/each}}";
        let data3 = json!({"items": [{"name": "A"}, {"name": "B"}]});
        let result3 = handlebars.render_template(template3, &data3).unwrap();
        assert_eq!(result3, "A B ");
    }

    #[test]
    fn test_unicode_support() {
        let unicode_data = json!({
            "chinese": "你好世界",
            "japanese": "こんにちは",
            "korean": "안녕하세요",
            "emoji": "🚀🦀💻",
            "mixed": "Hello 世界 🌍"
        });

        let serialized = serde_json::to_string(&unicode_data).unwrap();
        assert!(serialized.contains("你好世界"));
        assert!(serialized.contains("こんにちは"));
        assert!(serialized.contains("🚀"));
    }

    #[test] 
    fn test_template_validation() {
        use handlebars::Handlebars;
        
        let handlebars = Handlebars::new();
        
        // 有效模板
        let valid_templates = vec![
            "Hello {{name}}",
            "{{#if condition}}Yes{{/if}}",
            "{{#each items}}{{name}}{{/each}}",
            "{{#unless empty}}Not empty{{/unless}}"
        ];

        for template in valid_templates {
            let result = handlebars.render_template(template, &json!({}));
            assert!(result.is_ok(), "模板应该有效: {}", template);
        }
    }

    #[test]
    fn test_error_handling() {
        use handlebars::Handlebars;
        
        let handlebars = Handlebars::new();
        
        // 测试无效的 JSON 数据
        let invalid_json = "{ invalid json }";
        let parse_result = serde_json::from_str::<serde_json::Value>(invalid_json);
        assert!(parse_result.is_err());

        // 测试模板渲染错误场景
        let template = "{{nonexistent.property.chain}}";
        let data = json!({});
        let result = handlebars.render_template(template, &data);
        // 这应该不会出错，只是渲染为空字符串
        assert!(result.is_ok());
    }

    #[test]
    fn test_large_data_structure() {
        // 测试大型数据结构的处理
        let mut large_data = json!({
            "users": [],
            "total_count": 1000
        });

        // 生成大量测试数据
        let users = (0..100).map(|i| {
            json!({
                "id": i,
                "name": format!("用户{}", i),
                "email": format!("user{}@example.com", i),
                "active": i % 2 == 0,
                "tags": vec![format!("tag{}", i % 5), "common".to_string()]
            })
        }).collect::<Vec<_>>();

        large_data["users"] = json!(users);

        let serialized = serde_json::to_string(&large_data).unwrap();
        assert!(serialized.len() > 1000);
        assert!(serialized.contains("用户99"));
    }
}
