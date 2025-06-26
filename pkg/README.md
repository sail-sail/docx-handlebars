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

- ✅ 解析和处理 DOCX 文件
- ✅ Handlebars 模板引擎集成
- ✅ 支持复杂的模板语法（循环、条件等）
- ✅ 跨平台兼容性
- ✅ TypeScript 类型定义
- ✅ 零依赖的 WASM 二进制文件

## 安装

### Rust

```toml
[dependencies]
docx-handlebars = "0.1"
```

### npm

```bash
npm install docx-handlebars
```

### Deno

```typescript
import { DocxHandlebars } from "https://deno.land/x/docx_handlebars/mod.ts";
```

### JSR

```bash
npx jsr add @sail/docx-handlebars
```

## 使用示例

### Rust

```rust
use docx_handlebars::{DocxHandlebars, TemplateData};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut processor = DocxHandlebars::new();
    
    // 加载 DOCX 模板
    let template_bytes = std::fs::read("template.docx")?;
    processor.load_template(&template_bytes)?;
    
    // 准备数据
    let mut data = HashMap::new();
    data.insert("name".to_string(), "张三".into());
    data.insert("company".to_string(), "ABC公司".into());
    
    // 渲染模板
    let result = processor.render(&data)?;
    
    // 保存结果
    std::fs::write("output.docx", result)?;
    
    Ok(())
}
```

### JavaScript/TypeScript (Node.js)

```javascript
import { DocxHandlebars } from 'docx-handlebars';
import fs from 'fs';

async function processTemplate() {
    const processor = new DocxHandlebars();
    
    // 加载模板
    const templateBuffer = fs.readFileSync('template.docx');
    await processor.loadTemplate(templateBuffer);
    
    // 准备数据
    const data = {
        name: '张三',
        company: 'ABC公司',
        items: [
            { product: '产品A', price: 100 },
            { product: '产品B', price: 200 }
        ]
    };
    
    // 渲染模板
    const result = await processor.render(data);
    
    // 保存结果
    fs.writeFileSync('output.docx', result);
}

processTemplate().catch(console.error);
```

### 浏览器端

```html
<!DOCTYPE html>
<html>
<head>
    <title>DOCX Handlebars Demo</title>
</head>
<body>
    <input type="file" id="templateFile" accept=".docx">
    <button onclick="processTemplate()">处理模板</button>
    <a id="downloadLink" style="display:none">下载结果</a>

    <script type="module">
        import init, { DocxHandlebars } from './pkg/docx_handlebars.js';
        
        async function initWasm() {
            await init();
        }
        
        window.processTemplate = async function() {
            const fileInput = document.getElementById('templateFile');
            const file = fileInput.files[0];
            
            if (!file) {
                alert('请选择一个 DOCX 文件');
                return;
            }
            
            const arrayBuffer = await file.arrayBuffer();
            const bytes = new Uint8Array(arrayBuffer);
            
            const processor = new DocxHandlebars();
            processor.load_template(bytes);
            
            const data = {
                name: '张三',
                company: 'ABC公司'
            };
            
            const result = processor.render(JSON.stringify(data));
            
            // 创建下载链接
            const blob = new Blob([result], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
            const url = URL.createObjectURL(blob);
            const link = document.getElementById('downloadLink');
            link.href = url;
            link.download = 'output.docx';
            link.style.display = 'block';
            link.textContent = '下载处理后的文档';
        };
        
        initWasm();
    </script>
</body>
</html>
```

### Deno

```typescript
import { DocxHandlebars } from "https://deno.land/x/docx_handlebars/mod.ts";

const processor = new DocxHandlebars();

// 加载模板
const templateBytes = await Deno.readFile("template.docx");
await processor.loadTemplate(templateBytes);

// 渲染数据
const data = {
    name: "张三",
    company: "ABC公司"
};

const result = await processor.render(data);

// 保存结果
await Deno.writeFile("output.docx", result);
```

## 模板语法

支持完整的 Handlebars 语法：

```handlebars
{{name}} 在 {{company}} 工作

{{#if hasItems}}
产品列表：
{{#each items}}
- {{product}}: ¥{{price}}
{{/each}}
{{/if}}

{{#unless isEmpty}}
总计: ¥{{total}}
{{/unless}}
```

## 开发

### 前置条件

- Rust 1.70+
- wasm-pack
- Node.js 16+

### 构建

```bash
# 构建 Rust 库
cargo build --release

# 构建 WASM 包
wasm-pack build --target web --out-dir pkg

# 构建 npm 包
wasm-pack build --target nodejs --out-dir pkg-node

# 运行测试
cargo test
wasm-pack test --headless --firefox
```

### 发布

```bash
# 发布到 crates.io
cargo publish

# 发布到 npm
cd pkg && npm publish

# 发布到 JSR
deno publish
```

## 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 更新日志

### 0.1.0
- 初始版本
- 基本的 DOCX 模板处理功能
- 支持多平台部署
