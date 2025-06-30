// Node.js example for docx-handlebars
const fs = require("node:fs");
const path = require("node:path");
const { fileURLToPath } = require("node:url");

// 注意：这个示例需要 WASM 包构建完成后才能运行
// 运行 `npm run build:npm` 来构建 npm 包

async function nodeExample() {
    try {
        // 导入 WASM 模块 - 使用新的函数式 API
        const { default: init, render } = await import('../pkg-npm/docx_handlebars.js');
        
        // 指定 WASM 文件路径
        const wasmPath = path.join(__dirname, '../pkg-npm/docx_handlebars_bg.wasm');
        const wasmBuffer = fs.readFileSync(wasmPath);
        await init(wasmBuffer);
        
        console.log('🚀 Node.js DOCX Handlebars 处理示例\n');
        
        // 示例：假设您有一个模板文件
        const templatePath = path.join(__dirname, 'template.docx');
        
        // 检查模板文件是否存在
        if (!fs.existsSync(templatePath)) {
            console.log('⚠️  模板文件不存在，创建示例说明...\n');
            return;
        }
        
        // 读取模板文件
        console.log('📖 读取模板文件...');
        const templateBuffer = fs.readFileSync(templatePath);
        
        console.log('⚙️  准备处理模板...');
        
        // 准备数据
        const data = {
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
            }
        };
        
        console.log('📋 使用的数据:');
        console.log(JSON.stringify(data, null, 2));
        
        // 渲染模板 - 使用新的函数式 API
        console.log('\n🎨 渲染模板...');
        const result = render(new Uint8Array(templateBuffer), JSON.stringify(data));
        
        // 保存结果
        const outputPath = path.join(__dirname, 'output_node.docx');
        fs.writeFileSync(outputPath, new Uint8Array(result));
        
        console.log(`✅ 处理完成！结果已保存到: ${outputPath}`);
        
        console.log('\n🎉 示例执行完成！');
        console.log('\n💡 提示: 新的函数式 API 更简洁，直接传入文件字节和数据即可！');
        
    } catch (error) {
        console.error('❌ 错误:', error.message || error);
        
        // 处理不同类型的错误
        if (error.message && error.message.includes('Cannot resolve module')) {
            console.log('\n💡 提示: 请先构建 Node.js 版本的 WASM 包:');
            console.log('   npm run build:npm');
        } else if (error.message && error.message.includes('文件大小不足')) {
            console.log('\n💡 提示: 上传的文件太小，不是有效的 DOCX 文件');
        } else if (error.message && error.message.includes('无效的 ZIP 签名')) {
            console.log('\n💡 提示: 文件不是有效的 ZIP/DOCX 格式');
        } else if (error.message && error.message.includes('缺少必需的 DOCX 文件')) {
            console.log('\n💡 提示: 文件不包含必需的 DOCX 组件');
        }
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
