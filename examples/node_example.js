// Node.js example for docx-handlebars
const fs = require("node:fs");
const path = require("node:path");

// Note: This example requires the WASM package to be built first
// Run `npm run build:npm` to build the npm package

async function nodeExample() {
    try {
        // Import WASM module - using new functional API
        const { default: init, render_template } = await import('../pkg-npm/docx_handlebars.js');
        
        // Specify WASM file path
        const wasmPath = path.join(__dirname, '../pkg-npm/docx_handlebars_bg.wasm');
        const wasmBuffer = fs.readFileSync(wasmPath);
        await init(wasmBuffer);
        
        console.log('🚀 Node.js XLSX Handlebars Processing Example\n');
        
        // Example: Assume you have a template file
        const templatePath = path.join(__dirname, 'template.docx');
        
        // Check if template file exists
        if (!fs.existsSync(templatePath)) {
            console.log('⚠️  Template file does not exist, creating example description...\n');
            return;
        }
        
        // Read template file
        console.log('📖 Reading template file...');
        const templateBuffer = fs.readFileSync(templatePath);
        
        console.log('⚙️  Preparing to process template...');
        
        const imageBase64 = fs.readFileSync(__dirname + '/image.webp').toString('base64');
        const image_anchor = fs.readFileSync(__dirname + '/label_adopt.png').toString('base64');
        
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
        console.log('\n🎨 Rendering template...');
        const result = render_template(new Uint8Array(templateBuffer), JSON.stringify(data));
        
        // Save result
        const outputPath = path.join(__dirname, 'output_node.docx');
        fs.writeFileSync(outputPath, new Uint8Array(result));
        
        console.log(`✅ Processing complete! Result saved to: ${outputPath}`);
        
        console.log('\n🎉 Example completed successfully!');
        console.log('\n💡 Tip: The new functional API is simpler - just pass file bytes and data directly!');
        
    } catch (error) {
        console.error('❌ Error:', error.message || error);
    }
}

// 运行示例
if (require.main === module) {
    nodeExample()
        .catch(console.error);
}

module.exports = {
    nodeExample,
};
