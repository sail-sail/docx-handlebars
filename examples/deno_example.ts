/**
 * Deno example for docx-handlebars
 * 
 * 运行命令: deno run --allow-read --allow-write examples/deno_example.ts
 */

// 注意：这需要先构建 WASM 包
// 运行 `wasm-pack build --target web --out-dir pkg` 来构建包

import { DocxHandlebarsUtils } from "../mod.ts";

async function denoExample() {
    console.log("🦕 Deno DOCX Handlebars 处理示例\n");
    
    try {
        // 初始化 WASM 模块
        console.log("⚡ 初始化 WASM 模块...");
        await DocxHandlebarsUtils.initWasm();
        
        // 导入 DocxHandlebars 类
        const { DocxHandlebars } = await import("../pkg/docx_handlebars.js");
        
        // 创建处理器实例
        const processor = new DocxHandlebars();
        
        // 检查模板文件是否存在
        const templatePath = "./examples/template.docx";
        const templateExists = await DocxHandlebarsUtils.fileExists(templatePath);
        
        if (!templateExists) {
            console.log("⚠️  模板文件不存在，创建示例说明...\n");
            console.log("要使用此示例，请：");
            console.log("1. 创建一个包含以下内容的 DOCX 文件:");
            console.log("   员工姓名: {{employee.name}}");
            console.log("   部门: {{employee.department}}");
            console.log("   入职日期: {{employee.hire_date}}");
            console.log("   \n   项目经历:");
            console.log("   {{#each projects}}");
            console.log("   - {{name}}: {{description}} ({{status}})");
            console.log("   {{/each}}");
            console.log("   \n   {{#if employee.has_bonus}}");
            console.log("   奖金: ¥{{employee.bonus_amount}}");
            console.log("   {{/if}}");
            console.log("\n2. 将文件保存为 examples/template.docx");
            console.log("3. 重新运行此示例");
            console.log("\n💡 运行命令: deno run --allow-read --allow-write examples/deno_example.ts\n");
            return;
        }
        
        // 读取模板文件
        console.log("📖 读取模板文件...");
        const templateBytes = await DocxHandlebarsUtils.readDocxFile(templatePath);
        
        // 加载模板
        console.log("⚙️  加载模板...");
        processor.load_template(templateBytes);
        
        // 准备数据
        const data = {
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
        
        console.log("📋 使用的数据:");
        console.log(JSON.stringify(data, null, 2));
        
        // 提取模板变量
        console.log("\n🔍 提取模板变量...");
        const variablesJson = processor.get_template_variables();
        const variables = JSON.parse(variablesJson);
        console.log("发现的模板变量:", variables);
        
        // 渲染模板
        console.log("\n🎨 渲染模板...");
        const result = processor.render(JSON.stringify(data));
        
        // 保存结果
        const outputPath = "./examples/output_deno.docx";
        await DocxHandlebarsUtils.writeDocxFile(outputPath, result);
        
        console.log(`✅ 处理完成！结果已保存到: ${outputPath}`);
        console.log(`📁 文件大小: ${result.length} 字节`);
        
        // 额外的实用功能演示
        await demonstrateUtilities();
        
        console.log("\n🎉 Deno 示例执行完成！");
        
    } catch (error) {
        if (error instanceof Error) {
            console.error("❌ 错误:", error.message);

            if (error.message.includes("Cannot resolve")) {
                console.log("\n💡 提示: 请先构建 WASM 包:");
                console.log("   wasm-pack build --target web --out-dir pkg");
            }
        } else {
            console.error("❌ 错误:", error);
        }
    }
}

async function demonstrateUtilities() {
    console.log("\n🛠️  实用工具演示:");
    
    // 文件操作演示
    const testFiles = [
        "./examples/template.docx",
        "./examples/output_deno.docx",
        "./examples/nonexistent.docx"
    ];
    
    for (const file of testFiles) {
        const exists = await DocxHandlebarsUtils.fileExists(file);
        console.log(`📄 ${file}: ${exists ? "✅ 存在" : "❌ 不存在"}`);
    }
    
    // JSON 验证演示
    const testJsons = [
        '{"valid": "json"}',
        '{valid: "json"}', // 无效
        '{"name": "测试", "value": 123}',
        'invalid json'
    ];
    
    console.log("\n📝 JSON 验证演示:");
    testJsons.forEach((json, index) => {
        try {
            JSON.parse(json);
            console.log(`${index + 1}. ✅ 有效 JSON: ${json.slice(0, 30)}...`);
        } catch {
            console.log(`${index + 1}. ❌ 无效 JSON: ${json.slice(0, 30)}...`);
        }
    });
}

// Deno 特有的模板数据生成器
function generateDenoSpecificData() {
    return {
        runtime: {
            name: "Deno",
            version: "1.40+",
            features: ["TypeScript", "安全默认", "内置工具", "Web API"]
        },
        permissions: {
            read: true,
            write: true,
            net: false,
            env: false
        },
        timestamp: new Date().toISOString(),
        platform: Deno.build,
        // memory_usage: `${Math.round(performance.memory?.usedJSHeapSize / 1024 / 1024 || 0)}MB`
    };
}

// 错误处理示例
async function errorHandlingExample() {
    console.log("\n🚨 错误处理示例:");
    
    try {
        // 故意使用无效的 JSON
        const processor = new (await import("../pkg/docx_handlebars.js")).DocxHandlebars();
        processor.render("invalid json");
    } catch (error) {
        if (error instanceof Error) {
            console.log("✅ 成功捕获错误:", error.message);
        } else {
            console.log("✅ 成功捕获错误:", error);
        }
    }
    
    try {
        // 尝试读取不存在的文件
        await DocxHandlebarsUtils.readDocxFile("./nonexistent.docx");
    } catch (error) {
        if (typeof error === "object" && error !== null && "name" in error) {
            console.log("✅ 成功捕获文件读取错误:", (error as { name: string }).name);
        } else {
            console.log("✅ 成功捕获文件读取错误:", error);
        }
    }
}

// 性能测试示例
function performanceExample() {
    console.log("\n⚡ 性能测试示例:");
    
    const iterations = 100;
    const testData = { test: "data", number: 42, array: [1, 2, 3] };
    
    const start = performance.now();
    
    for (let i = 0; i < iterations; i++) {
        JSON.stringify(testData);
        JSON.parse(JSON.stringify(testData));
    }
    
    const end = performance.now();
    console.log(`⏱️  ${iterations} 次 JSON 序列化/反序列化耗时: ${(end - start).toFixed(2)}ms`);
}

// 主函数
if (import.meta.main) {
    await denoExample();
    await errorHandlingExample();
    performanceExample();
}
