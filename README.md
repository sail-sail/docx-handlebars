# docx-handlebars

[![Crates.io](https://img.shields.io/crates/v/docx-handlebars.svg)](https://crates.io/crates/docx-handlebars)
[![Documentation](https://docs.rs/docx-handlebars/badge.svg)](https://docs.rs/docx-handlebars)
[![License](https://img.shields.io/crates/l/docx-handlebars.svg)](https://github.com/sail-sail/docx-handlebars#license)

一个用于处理 DOCX 文件 Handlebars 模板的 Rust 库，支持多平台使用：
- 🦀 Rust 原生
- 🌐 WebAssembly (WASM)
- 📦 npm 包
- 🟢 Node.js
- 🦕 Deno
- 🌍 浏览器端
- 📋 JSR (JavaScript Registry)

## 功能特性

- ✅ **智能合并**：自动处理被 XML 标签分割的 Handlebars 语法
- ✅ **DOCX 验证**：内置文件格式验证，确保输入文件有效
- ✅ **Handlebars 支持**：完整的模板引擎，支持变量、条件、循环、Helper 函数
- ✅ **跨平台**：Rust 原生 + WASM 支持多种运行时
- ✅ **TypeScript**：完整的类型定义和智能提示
- ✅ **零依赖**：WASM 二进制文件，无外部依赖
- ✅ **错误处理**：详细的错误信息和类型安全的错误处理

## 安装

### Rust

```bash
cargo add docx-handlebars
```

### npm

```bash
npm install docx-handlebars
```

### Deno

```typescript
import { render, init } from "https://deno.land/x/docx_handlebars/mod.ts";
```

### JSR

```bash
npx jsr add @sail/docx-handlebars
```

## 使用示例

### Rust

```rust
use docx_handlebars::render_handlebars;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 DOCX 模板文件
    let template_bytes = std::fs::read("template.docx")?;
    
    // 准备数据
    let data = json!({
        "name": "张三",
        "company": "ABC科技有限公司",
        "position": "软件工程师",
        "projects": [
            {"name": "项目A", "status": "已完成"},
            {"name": "项目B", "status": "进行中"}
        ],
        "has_bonus": true,
        "bonus_amount": 5000
    });
    
    // 渲染模板
    let result = render_handlebars(template_bytes, &data)?;
    
    // 保存结果
    std::fs::write("output.docx", result)?;
    
    Ok(())
}
```

### JavaScript/TypeScript (Node.js)

```javascript
import { render, init } from 'docx-handlebars';
import fs from 'fs';

async function processTemplate() {
    // 初始化 WASM 模块
    await init();
    
    // 读取模板文件
    const templateBytes = fs.readFileSync('template.docx');
    
    // 准备数据
    const data = {
        name: "李明",
        company: "XYZ技术有限公司",
        position: "高级开发工程师",
        projects: [
            { name: "E-commerce平台", status: "已完成" },
            { name: "移动端APP", status: "开发中" }
        ],
        has_bonus: true,
        bonus_amount: 8000
    };
    
    // 渲染模板
    const result = render(templateBytes, JSON.stringify(data));
    
    // 保存结果
    fs.writeFileSync('output.docx', new Uint8Array(result));
}

processTemplate().catch(console.error);
```

### Deno

```typescript
import { render, init } from "https://deno.land/x/docx_handlebars/mod.ts";

async function processTemplate() {
    // 初始化 WASM 模块
    await init();
    
    // 读取模板文件
    const templateBytes = await Deno.readFile("template.docx");
    
    // 准备数据
    const data = {
        name: "王小明",
        department: "研发部",
        projects: [
            { name: "智能客服系统", status: "已上线" },
            { name: "数据可视化平台", status: "开发中" }
        ]
    };
    
    // 渲染模板
    const result = render(templateBytes, JSON.stringify(data));
    
    // 保存结果
    await Deno.writeFile("output.docx", new Uint8Array(result));
}

if (import.meta.main) {
    await processTemplate();
}
```

### 浏览器端

```html
<!DOCTYPE html>
<html>
<head>
    <title>DOCX Handlebars 示例</title>
</head>
<body>
    <input type="file" id="fileInput" accept=".docx">
    <button onclick="processFile()">处理模板</button>
    
    <script type="module">
        import { render, init } from './pkg/docx_handlebars.js';
        
        // 初始化 WASM
        await init();
        
        window.processFile = async function() {
            const fileInput = document.getElementById('fileInput');
            const file = fileInput.files[0];
            
            if (!file) return;
            
            const arrayBuffer = await file.arrayBuffer();
            const templateBytes = new Uint8Array(arrayBuffer);
            
            const data = {
                name: "张三",
                company: "示例公司"
            };
            
            try {
                const result = render(templateBytes, JSON.stringify(data));
                
                // 下载结果
                const blob = new Blob([new Uint8Array(result)], {
                    type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'
                });
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'processed.docx';
                a.click();
            } catch (error) {
                console.error('处理失败:', error);
            }
        };
    </script>
</body>
</html>
```

## 模板语法

### 基础变量替换

```handlebars
员工姓名: {{name}}
公司: {{company}}
职位: {{position}}
```

### 条件渲染

```handlebars
{{#if has_bonus}}
奖金: ¥{{bonus_amount}}
{{else}}
无奖金
{{/if}}

{{#unless is_intern}}
正式员工
{{/unless}}
```

### 循环渲染

```handlebars
项目经历:
{{#each projects}}
- {{name}}: {{description}} ({{status}})
{{/each}}

技能列表:
{{#each skills}}
{{@index}}. {{this}}
{{/each}}
```

### Helper 函数

内置的 Helper 函数：

```handlebars
{{upper name}}           <!-- 转大写 -->
{{lower company}}        <!-- 转小写 -->
{{len projects}}         <!-- 数组长度 -->
{{#if (eq status "completed")}}已完成{{/if}}    <!-- 相等比较 -->
{{#if (gt score 90)}}优秀{{/if}}               <!-- 大于比较 -->
{{#if (lt age 30)}}年轻{{/if}}                 <!-- 小于比较 -->
```

### 复杂示例

```handlebars
=== 员工报告 ===

基本信息:
姓名: {{employee.name}}
部门: {{employee.department}}
职位: {{employee.position}}
入职时间: {{employee.hire_date}}

{{#if employee.has_bonus}}
💰 奖金: ¥{{employee.bonus_amount}}
{{/if}}

项目经历 (共{{len projects}}个):
{{#each projects}}
{{@index}}. {{name}}
   描述: {{description}}
   状态: {{status}}
   团队规模: {{team_size}}人
   
{{/each}}

技能评估:
{{#each skills}}
- {{name}}: {{level}}/10 ({{years}}年经验)
{{/each}}

{{#if (gt performance.score 90)}}
🎉 绩效评级: 优秀
{{else if (gt performance.score 80)}}
👍 绩效评级: 良好
{{else}}
📈 绩效评级: 需改进
{{/if}}
```

## 错误处理

库提供了详细的错误类型和消息：

### Rust

```rust
use docx_handlebars::{render_handlebars, DocxError};

match render_handlebars(template_bytes, &data) {
    Ok(result) => {
        println!("处理成功！");
        std::fs::write("output.docx", result)?;
    }
    Err(e) => match e.downcast_ref::<DocxError>() {
        Some(DocxError::FileTooSmall) => {
            eprintln!("错误: 文件太小，不是有效的 DOCX 文件");
        }
        Some(DocxError::InvalidZipSignature) => {
            eprintln!("错误: 文件不是有效的 ZIP/DOCX 格式");
        }
        Some(DocxError::MissingRequiredFile(filename)) => {
            eprintln!("错误: 缺少必需的 DOCX 文件: {}", filename);
        }
        _ => {
            eprintln!("其他错误: {}", e);
        }
    }
}
```

### JavaScript/TypeScript

```javascript
try {
    const result = render(templateBytes, JSON.stringify(data));
    console.log('处理成功！');
} catch (error) {
    if (error.message.includes('文件大小不足')) {
        console.error('文件太小，不是有效的 DOCX 文件');
    } else if (error.message.includes('无效的 ZIP 签名')) {
        console.error('文件不是有效的 ZIP/DOCX 格式');
    } else if (error.message.includes('缺少必需的 DOCX 文件')) {
        console.error('文件不包含必需的 DOCX 组件');
    } else if (error.message.includes('模板渲染失败')) {
        console.error('Handlebars 模板语法错误或数据不匹配');
    } else {
        console.error('处理失败:', error.message);
    }
}
```

## 构建和开发

### 构建 WASM 包

```bash
# 构建所有目标
npm run build

# 或分别构建
npm run build:web    # 浏览器版本
npm run build:npm    # Node.js 版本 
npm run build:jsr    # Deno 版本
```

### 运行示例

```bash
# Rust 示例
cargo run --example rust_example

# Node.js 示例
node examples/node_example.js

# Deno 示例  
deno run --allow-read --allow-write examples/deno_example.ts

# 浏览器示例
# 启动本地服务器并打开 examples/browser_demo.html
```

## 技术特性

### 智能合并算法

该库的核心创新是智能合并被 XML 标签分割的 Handlebars 语法。在 DOCX 文件中，当用户输入模板语法时，Word 可能会将其拆分成多个 XML 标签：

**原始分割状态：**
```xml
<w:t>员工姓名: {{</w:t><w:t>employee.name</w:t><w:t>}}</w:t>
```

**智能合并后：**
```xml
<w:t>员工姓名: {{employee.name}}</w:t>
```

支持的合并模式：
- 简单分割: `<w:t>{{</w:t><w:t>variable}}</w:t>`
- 部分分割: `<w:t>{{part1</w:t><w:t>part2}}</w:t>`
- 三段分割: `<w:t>{{</w:t><w:t>part1</w:t><w:t>part2}}</w:t>`
- 复杂嵌套: `<w:t>prefix{{</w:t><w:t>content</w:t><w:t>}}suffix</w:t>`

### 文件验证

内置的 DOCX 文件验证确保输入文件的完整性：

1. **ZIP 格式验证**：检查文件签名和结构
2. **DOCX 结构验证**：确保包含必要的文件
   - `[Content_Types].xml`
   - `_rels/.rels` 
   - `word/document.xml`
3. **MIME 类型验证**：验证内容类型正确性

## 性能和兼容性

- **零拷贝**: Rust 和 WASM 之间高效的内存管理
- **流式处理**: 适合处理大型 DOCX 文件
- **跨平台**: 支持 Windows、macOS、Linux、Web
- **现代浏览器**: 支持所有支持 WASM 的现代浏览器

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE-MIT](LICENSE-MIT) 文件。

## 贡献

欢迎贡献代码！请查看我们的贡献指南：

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 更新日志

### v0.1.6

- ✨ **重大重构**: 采用函数式 API，更简洁易用
- ✨ **智能合并**: 完善的 Handlebars 语法分割合并算法  
- ✨ **文件验证**: 内置 DOCX 格式验证和错误处理
- ✨ **错误处理**: 使用 thiserror 提供详细的错误信息
- ✨ **Helper 函数**: 内置常用的 Handlebars helper
- 🐛 **修复**: 多种边界情况和兼容性问题
- 📚 **文档**: 全面更新文档和示例
- 🧪 **测试**: 完整的测试覆盖和验证脚本

## 支持

- 📚 [文档](https://docs.rs/docx-handlebars)
- 🐛 [问题反馈](https://github.com/sail-sail/docx-handlebars/issues)
- 💬 [讨论](https://github.com/sail-sail/docx-handlebars/discussions)

---

<div align="center">
  <p>
    <strong>docx-handlebars</strong> - 让 DOCX 模板处理变得简单高效
  </p>
  <p>
    <a href="https://github.com/sail-sail/docx-handlebars">⭐ 给项目点个星</a>
    ·
    <a href="https://github.com/sail-sail/docx-handlebars/issues">🐛 报告问题</a>
    ·
    <a href="https://github.com/sail-sail/docx-handlebars/discussions">💬 参与讨论</a>
  </p>
</div>
