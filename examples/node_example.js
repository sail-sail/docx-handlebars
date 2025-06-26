// Node.js example for docx-handlebars
const fs = require("node:fs");
const path = require("node:path");

// 注意：这个示例需要 WASM 包构建完成后才能运行
// 运行 `wasm-pack build --target nodejs --out-dir pkg-node` 来构建 Node.js 包

async function nodeExample() {
    try {
        // 导入 WASM 模块
        const { DocxHandlebars } = require('../pkg-node/docx_handlebars.js');
        
        console.log('🚀 Node.js DOCX Handlebars 处理示例\n');
        
        // 创建处理器实例
        const processor = new DocxHandlebars();
        
        // 示例：假设您有一个模板文件
        const templatePath = path.join(__dirname, 'template.docx');
        
        // 检查模板文件是否存在
        if (!fs.existsSync(templatePath)) {
            console.log('⚠️  模板文件不存在，创建示例说明...\n');
            console.log('要使用此示例，请：');
            console.log('1. 创建一个包含以下内容的 DOCX 文件:');
            console.log('   姓名: {{name}}');
            console.log('   公司: {{company}}');
            console.log('   职位: {{position}}');
            console.log('   {{#if has_bonus}}奖金: ¥{{bonus_amount}}{{/if}}');
            console.log('   \n   项目列表:');
            console.log('   {{#each projects}}');
            console.log('   - {{name}}: {{description}} ({{status}})');
            console.log('   {{/each}}');
            console.log('\n2. 将文件保存为 examples/template.docx');
            console.log('3. 重新运行此示例\n');
            return;
        }
        
        // 读取模板文件
        console.log('📖 读取模板文件...');
        const templateBuffer = fs.readFileSync(templatePath);
        
        // 加载模板
        console.log('⚙️  加载模板...');
        processor.load_template(new Uint8Array(templateBuffer));
        
        // 准备数据
        const data = {
            name: '李明',
            company: 'XYZ技术有限公司',
            position: '高级开发工程师',
            salary: 18000,
            start_date: '2024-03-01',
            has_bonus: true,
            bonus_amount: 8000,
            projects: [
                {
                    name: 'E-commerce平台',
                    description: '在线购物系统开发',
                    status: '已完成'
                },
                {
                    name: '移动端APP',
                    description: 'React Native应用',
                    status: '开发中'
                },
                {
                    name: '数据分析系统',
                    description: '业务数据可视化',
                    status: '规划中'
                }
            ],
            skills: ['JavaScript', 'React', 'Node.js', 'Python', 'Rust'],
            contact: {
                email: 'liming@xyz.com',
                phone: '138-0000-0000'
            }
        };
        
        console.log('📋 使用的数据:');
        console.log(JSON.stringify(data, null, 2));
        
        // 渲染模板
        console.log('\n🎨 渲染模板...');
        const result = processor.render(JSON.stringify(data));
        
        // 保存结果
        const outputPath = path.join(__dirname, 'output_node.docx');
        fs.writeFileSync(outputPath, result);
        
        console.log(`✅ 处理完成！结果已保存到: ${outputPath}`);
        
        // 提取模板变量
        console.log('\n🔍 提取模板变量...');
        const variablesJson = processor.get_template_variables();
        const variables = JSON.parse(variablesJson);
        console.log('模板变量:', variables);
        
        console.log('\n🎉 示例执行完成！');
        
    } catch (error) {
        console.error('❌ 错误:', error.message);
        
        if (error.message.includes('Cannot resolve module')) {
            console.log('\n💡 提示: 请先构建 Node.js 版本的 WASM 包:');
            console.log('   wasm-pack build --target nodejs --out-dir pkg-node');
        }
    }
}

// 异步函数调用示例
async function advancedExample() {
    console.log('\n📚 高级使用示例\n');
    
    // 模拟更复杂的数据处理
    const templateData = {
        // 公司信息
        company: {
            name: 'TechCorp 科技有限公司',
            address: '北京市朝阳区科技园区100号',
            phone: '010-12345678',
            website: 'https://techcorp.com'
        },
        
        // 员工信息
        employee: {
            id: 'E001',
            name: '王小明',
            department: '研发部',
            position: '资深工程师',
            hire_date: '2023-06-15',
            salary: 20000,
            performance_rating: 'A'
        },
        
        // 项目经历
        projects: [
            {
                name: '智能客服系统',
                duration: '2023-07 至 2023-12',
                role: '技术负责人',
                technologies: ['Python', 'FastAPI', 'PostgreSQL', 'Redis'],
                achievements: [
                    '提升客服响应速度50%',
                    '减少人工处理工作量30%',
                    '获得用户满意度95%'
                ]
            },
            {
                name: '数据可视化平台',
                duration: '2024-01 至今',
                role: '核心开发者',
                technologies: ['React', 'D3.js', 'Node.js', 'MongoDB'],
                achievements: [
                    '支持10+种图表类型',
                    '实时数据处理能力',
                    '获得产品创新奖'
                ]
            }
        ],
        
        // 技能评估
        skills: [
            { name: 'JavaScript', level: 9, years: 5 },
            { name: 'Python', level: 8, years: 4 },
            { name: 'Rust', level: 7, years: 2 },
            { name: 'React', level: 9, years: 3 },
            { name: 'Node.js', level: 8, years: 4 }
        ],
        
        // 统计信息
        stats: {
            total_projects: 15,
            completed_projects: 12,
            in_progress: 3,
            total_lines_of_code: 50000,
            bugs_fixed: 234,
            code_reviews: 156
        },
        
        // 条件标志
        flags: {
            is_senior: true,
            has_leadership_experience: true,
            eligible_for_promotion: true,
            has_certifications: true
        },
        
        // 时间相关
        current_date: new Date().toISOString().split('T')[0],
        report_period: '2024年第一季度'
    };
    
    console.log('📊 复杂数据结构示例:');
    console.log(JSON.stringify(templateData, null, 2));
    
    console.log('\n📝 对应的模板语法示例:');
    console.log(`
员工报告 - {{report_period}}

员工信息:
姓名: {{employee.name}}
工号: {{employee.id}}
部门: {{employee.department}}
职位: {{employee.position}}

公司信息:
{{company.name}}
地址: {{company.address}}
联系电话: {{company.phone}}

项目经历:
{{#each projects}}
{{@index}}. {{name}} ({{duration}})
   角色: {{role}}
   技术栈: {{#each technologies}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}
   成就:
   {{#each achievements}}
   - {{this}}
   {{/each}}
{{/each}}

技能评估:
{{#each skills}}
{{name}}: {{level}}/10 ({{years}}年经验)
{{/each}}

统计数据:
- 总项目数: {{stats.total_projects}}
- 已完成: {{stats.completed_projects}}
- 进行中: {{stats.in_progress}}
- 代码行数: {{stats.total_lines_of_code}}

{{#if flags.eligible_for_promotion}}
✅ 符合晋升条件
{{/if}}

{{#if flags.has_certifications}}
🏆 拥有专业认证
{{/if}}

报告生成时间: {{current_date}}
    `);
}

// 运行示例
if (require.main === module) {
    nodeExample()
        .then(() => advancedExample())
        .catch(console.error);
}

module.exports = {
    nodeExample,
    advancedExample
};
