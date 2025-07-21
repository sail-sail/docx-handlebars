use base64::{engine::general_purpose, Engine};
/**
 * Rust native example for docx-handlebars
 * 
 * 运行命令: cargo run --example rust_example
 */

use docx_handlebars::template;
use serde_json::json;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Rust Native DOCX Handlebars 处理示例\n");
    
    // 检查模板文件是否存在
    let template_path = "./examples/template.docx";
    if !std::path::Path::new(template_path).exists() {
        println!("⚠️  模板文件不存在: {}", template_path);
        println!("要使用此示例，请：");
        println!("1. 创建一个包含以下内容的 DOCX 文件:");
        println!("   员工姓名: {{{{employee.name}}}}");
        println!("   部门: {{{{employee.department}}}}");
        println!("   入职日期: {{{{employee.hire_date}}}}");
        println!("   \\n   项目经历:");
        println!("   {{{{#each projects}}}}");
        println!("   - {{{{name}}}}: {{{{description}}}} ({{{{status}}}})");
        println!("   {{{{/each}}}}");
        println!("   \\n   {{{{#if employee.has_bonus}}}}");
        println!("   奖金: ¥{{{{employee.bonus_amount}}}}");
        println!("   {{{{/if}}}}");
        println!("\\n2. 将文件保存为 examples/template.docx");
        return Ok(());
    }
    
    // 读取模板文件
    println!("📖 读取模板文件...");
    let template_bytes = fs::read(template_path)?;
    
    // 准备数据
    let data = json!({
        "employee": {
            "name": "陈小华",
            "department": "产品部",
            "position": "产品经理",
            "hire_date": "2024-02-20",
            "has_bonus": true,
            "bonus_amount": 12000,
            "email": "chenxiaohua@company.com"
        },
        "company": {
            "name": "创新科技有限公司",
            "address": "上海市浦东新区张江高科技园区",
            "industry": "人工智能"
        },
        "projects": [
            {
                "name": "AI助手平台",
                "description": "智能对话系统产品设计",
                "status": "已上线",
                "duration": "3个月",
                "team_size": 8
            },
            {
                "name": "数据分析工具",
                "description": "用户行为分析平台",
                "status": "开发中",
                "duration": "2个月",
                "team_size": 5
            },
            {
                "name": "移动应用重构",
                "description": "用户体验优化项目",
                "status": "规划中",
                "duration": "4个月",
                "team_size": 12
            }
        ],
        "skills": ["产品设计", "用户研究", "数据分析", "项目管理", "敏捷开发"],
        "achievements": [
            "产品用户量增长200%",
            "用户满意度提升至4.8/5.0",
            "获得年度最佳产品奖",
            "主导3次成功的产品迭代"
        ],
        "performance": {
            "rating": "优秀",
            "score": 92,
            "goals_achieved": 8,
            "total_goals": 10
        },
        "metadata": {
            "report_date": "2025/6/26",
            "quarter": "2024 Q1",
            "version": "v1.0"
        },
        "image": {
            "base64": general_purpose::STANDARD.encode(&fs::read("./examples/image.webp")?)
        }
    });
    
    // 渲染模板
    println!("\n🎨 渲染模板...");
    let result_bytes = template::render_handlebars(template_bytes, &data)?;
    
    // 保存结果
    let output_path = "./examples/output_rust.docx";
    fs::write(output_path, result_bytes)?;
    
    println!("✅ 处理完成！结果已保存到: {}", output_path);
    
    println!("\n🎉 Rust 示例执行完成！");
    
    Ok(())
}
