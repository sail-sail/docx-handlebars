// Extended test script to verify npm package functionality using ES modules
import init, { render } from 'docx-handlebars';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function testDocumentProcessing() {
  try {
    console.log('🧪 Testing docx-handlebars...\n');
    
    // Initialize WASM module using sync method with explicit WASM file
    const wasmPath = path.join(__dirname, 'node_modules', 'docx-handlebars', 'docx_handlebars_bg.wasm');
    const wasmBytes = fs.readFileSync(wasmPath);
    await init(wasmBytes);
    console.log('✓ WASM module initialized synchronously');
    
    console.log('✓ render function imported');
    
    // Test template file path
    const templatePath = path.join(__dirname, '..', '..', 'examples', 'template.docx');
    console.log('📄 Template path:', templatePath);
    
    // Check if template file exists
    if (!fs.existsSync(templatePath)) {
      console.log('⚠️  Template file not found, skipping document processing test');
      return;
    }
    
    // Load template
    const templateData = fs.readFileSync(templatePath);
    console.log('✓ Template file loaded, size:', templateData.length, 'bytes');
    
    // Test data for rendering
    const testData = {
        employee: {
            name: "陈小华",
            department: "产品部",
            position: "产品经理",
            hire_date: "2024-02-20",
            has_bonus: true,
            bonus_amount: 12000,
            email: "chenxiaohua@company.com"
        },
        company: {
            name: "创新科技有限公司",
            address: "上海市浦东新区张江高科技园区",
            industry: "人工智能"
        },
        projects: [
            {
                name: "AI助手平台",
                description: "智能对话系统产品设计",
                status: "已上线",
                duration: "3个月",
                team_size: 8
            },
            {
                name: "数据分析工具",
                description: "用户行为分析平台",
                status: "开发中",
                duration: "2个月",
                team_size: 5
            },
            {
                name: "移动应用重构",
                description: "用户体验优化项目",
                status: "规划中",
                duration: "4个月",
                team_size: 12
            }
        ],
        skills: ["产品设计", "用户研究", "数据分析", "项目管理", "敏捷开发"],
        achievements: [
            "产品用户量增长200%",
            "用户满意度提升至4.8/5.0",
            "获得年度最佳产品奖",
            "主导3次成功的产品迭代"
        ],
        performance: {
            rating: "优秀",
            score: 92,
            goals_achieved: 8,
            total_goals: 10
        },
        metadata: {
            report_date: new Date().toLocaleDateString("zh-CN"),
            quarter: "2024 Q1",
            version: "v1.0"
        }
    };
    
    // Render document using new functional API
    console.log('🔄 Rendering document with test data...');
    const renderedData = render(new Uint8Array(templateData), JSON.stringify(testData));
    console.log('✓ Document rendered successfully, size:', renderedData.length, 'bytes');
    
    // Save output
    const outputPath = path.join(__dirname, 'test_output_npm.docx');
    fs.writeFileSync(outputPath, new Uint8Array(renderedData));
    console.log('✓ Output saved to:', outputPath);
    
    console.log('\n🎉 npm package test completed successfully!');
    console.log('📋 Test Summary:');
    console.log('   - WASM initialization: ✓');
    console.log('   - Function import: ✓');
    console.log('   - Template loading: ✓');
    console.log('   - Document rendering: ✓');
    console.log('   - Output generation: ✓');
    
  } catch (error) {
    console.error('❌ npm package test failed:', error.message);
    console.error('Stack trace:', error.stack);
  }
}

testDocumentProcessing();
