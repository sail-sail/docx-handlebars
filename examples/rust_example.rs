use base64::{engine::general_purpose, Engine};
/**
 * Rust native example for docx-handlebars
 * 
 * 运行命令: cargo run --example rust_example
 */

use docx_handlebars::render_template;
use serde_json::json;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Rust Native DOCX Handlebars Processing Example\n");
    
    // Check if template file exists
    let template_path = "./examples/template.docx";
    if !std::path::Path::new(template_path).exists() {
        println!("⚠️  Template file does not exist: {}", template_path);
        return Ok(());
    }
    
    // Read template file
    println!("📖 读取模板文件...");
    let template_bytes = fs::read(template_path)?;
    
    // Prepare data
    let data = json!({
        "employee": {
            "name": "John Smith",
            "department": "Product",
            "position": "Product Manager",
            "hire_date": "2024-02-20",
            "has_bonus": true,
            "bonus_amount": 12000,
            "email": "john.smith@company.com"
        },
        "company": {
            "name": "Innovative Tech Ltd.",
            "address": "123 Tech Park, Silicon Valley, CA",
            "industry": "Artificial Intelligence"
        },
        "projects": [
            {
                "name": "AI Assistant Platform",
                "description": "Intelligent dialogue system product design",
                "status": "Launched",
                "duration": "3 months",
                "team_size": 8
            },
            {
                "name": "Data Analysis Tool",
                "description": "User behavior analysis platform",
                "status": "In Development",
                "duration": "2 months",
                "team_size": 5
            },
            {
                "name": "Mobile App Refactoring",
                "description": "User experience optimization project",
                "status": "Planning",
                "duration": "4 months",
                "team_size": 12
            }
        ],
        "skills": ["Product Design", "User Research", "Data Analysis", "Project Management", "Agile Development"],
        "achievements": [
            "Increased product user base by 200%",
            "Improved user satisfaction to 4.8/5.0",
            "Won Product of the Year Award",
            "Led 3 successful product iterations"
        ],
        "performance": {
            "rating": "Excellent",
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
            "base64": general_purpose::STANDARD.encode(&fs::read("./examples/image.webp")?),
            "image_anchor": general_purpose::STANDARD.encode(&fs::read("./examples/label_adopt.png")?),
            "options": {
                "anchor": true, // Whether to use anchor point positioning
                "position_h": 40, // Horizontal offset, in pixels (negative values indicate leftward offset)
                "position_v": -14 // Vertical offset, in pixels (negative values indicate upward offset)
            }
        }
    });
    
    // Render template
    println!("\n🎨 Rendering template...");
    let result_bytes = render_template(template_bytes, &data)?;
    
    // Save result
    let output_path = "./examples/output_rust.docx";
    fs::write(output_path, result_bytes)?;
    
    println!("✅ Processing complete! Result saved to: {}", output_path);
    
    println!("\n🎉 Rust example completed successfully!");
    
    Ok(())
}
