/**
 * Deno example for docx-handlebars
 * 
 * Run command: deno run --allow-read --allow-write examples/deno_example.ts
 */

// Note: This requires building the WASM package first
// Run `npm run build:jsr` to build the package

import init, { render_template } from "../pkg-jsr/mod.ts";

import * as fs from "node:fs";

async function denoExample() {
    console.log("🦕 Deno XLSX Handlebars Processing Example\n");
    
    try {
        
        // Initialize WASM module
        console.log("⚡ Initializing WASM module...");
        await init();
        
        // Check if template file exists
        const templatePath = "./examples/template.docx";
        
        let templateExists = true;
        
        try {
            await Deno.stat(templatePath);
        } catch (err) {
            templateExists = false;
        }
        
        if (!templateExists) {
            console.log("⚠️  Template file does not exist, creating example description...\n");
            console.log("To use this example, please:");
            return;
        }
        
        // Read template file
        console.log("📖 Reading template file...");
        const templateBytes = await Deno.readFile(templatePath);
        
        console.log("⚙️  Preparing to process template...");
        
        const imageBytes = await Deno.readFile('./examples/image.webp');
        const imageBase64 = btoa(Array.from(imageBytes, byte => String.fromCharCode(byte)).join(''));
        
        const image_anchorBytes = await Deno.readFile('./examples/label_adopt.png');
        const image_anchor = btoa(Array.from(image_anchorBytes, byte => String.fromCharCode(byte)).join(''));

        
        // Prepare data
        const data = {
            employee: {
                name: "John Smith",
                department: "Product",
                position: "Product Manager",
                hire_date: "2024-02-20",
                has_bonus: true,
                bonus_amount: 12000,
                email: "john.smith@company.com"
            },
            company: {
                name: "Innovative Tech Ltd.",
                address: "123 Tech Park, Silicon Valley, CA",
                industry: "Artificial Intelligence"
            },
            projects: [
                {
                    name: "AI Assistant Platform",
                    description: "Intelligent dialogue system product design",
                    status: "Launched",
                    duration: "3 months",
                    team_size: 8
                },
                {
                    name: "Data Analysis Tool",
                    description: "User behavior analysis platform",
                    status: "In Development",
                    duration: "2 months",
                    team_size: 5
                },
                {
                    name: "Mobile App Refactoring",
                    description: "User experience optimization project",
                    status: "Planning",
                    duration: "4 months",
                    team_size: 12
                }
            ],
            skills: ["Product Design", "User Research", "Data Analysis", "Project Management", "Agile Development"],
            achievements: [
                "Increased product user base by 200%",
                "Improved user satisfaction to 4.8/5.0",
                "Won Product of the Year Award",
                "Led 3 successful product iterations"
            ],
            performance: {
                rating: "Excellent",
                score: 92,
                goals_achieved: 8,
                total_goals: 10
            },
            metadata: {
                report_date: new Date().toLocaleDateString("en-US"),
                quarter: "2024 Q1",
                version: "v1.0"
            },
            image: {
                "base64": imageBase64, // Base64 encoded image
                "image_anchor": image_anchor, // Anchor Base64 encoded image
                "options": {
                    "anchor": true, // Whether to use anchor point positioning
                    "position_h": 40, // Horizontal offset, in pixels (negative values indicate leftward offset)
                    "position_v": -14 // Vertical offset, in pixels (negative values indicate upward offset)
                }
            },
        };
        
        // Render template - using new functional API
        console.log("\n🎨 Rendering template...");
        const result = render_template(templateBytes, JSON.stringify(data));
        
        // 保存结果
        const outputPath = "./examples/output_deno.docx";
        await Deno.writeFile(outputPath, new Uint8Array(result));
        
        console.log(`✅ Processing complete! Result saved to: ${outputPath}`);
        console.log(`📁 File size: ${result.length} bytes`);
        
        console.log("\n🎉 Deno example completed successfully!");
        console.log("💡 Tip: The new functional API is simpler - just pass file bytes and data directly!");
        
    } catch (error) {
        console.error("❌ Error:", error);
    }
}

// 主函数
if (import.meta.main) {
    await denoExample();
}
