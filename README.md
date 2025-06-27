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

```bash
cargo add docx-handlebars
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
        import init, { DocxHandlebars } from './pkg-npm/docx_handlebars.js';
        
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

## 项目结构

```
docx-handlebars/
├── src/              # Rust 源代码
├── examples/         # 使用示例
├── tests/            # 集成测试
│   ├── jsr_test/     # JSR 包测试
│   └── npm_test/     # npm 包测试
├── tools/            # Python 调试工具
├── releases/         # 发版说明文档
├── pkg-npm/          # npm 包构建输出
└── pkg-jsr/          # JSR 包构建输出
```

### 调试工具

`tools/` 目录包含用于调试和分析DOCX文件的Python工具：

- `check_template.py` - 检查DOCX文件内容
- `debug_extract.py` - 调试文本提取过程
- `debug_lines.py` - 调试渲染后文本的行分布
- `debug_specific.py` - 特定DOCX文件调试
- `debug_template.py` - 分析模板文件的段落结构

详细使用说明请参考 `tools/README.md`。

### 发版包目录

`pkg-npm/` 目录包含用于 npm 发版的包：
- 支持 Node.js 和浏览器环境
- 使用 web target 构建

`pkg-jsr/` 目录包含用于 JSR 发版的包：
- 支持 Deno 和 Node.js 环境  
- 包含 JSR 特定的配置文件

### 发版文档

`releases/` 目录包含项目的发版说明和版本历史：

- `FINAL_RELEASE_SUMMARY_0.1.4.md` - v0.1.4 最终发版总结
- `JSR_RELEASE_0.1.4_SUMMARY.md` - JSR 平台发版说明
- `MULTI_PLATFORM_RELEASE_0.1.4.md` - 多平台发版详情
- `RELEASE_SUMMARY.md` - 通用发版总结

详细信息请参考 `releases/README.md`。

## 开发

### 前置条件

- Rust 1.70+
- wasm-pack
- Node.js 16+

### 构建

```bash
# 构建所有包（推荐）
npm run build

# 或者分别构建：
# 构建 Rust 库
cargo build --release

# 单独构建各平台包
npm run build:npm  # 构建 npm 包
npm run build:jsr  # 构建 JSR 包

# 运行测试
cargo test
wasm-pack test --headless --firefox
```

### 发布

```bash
# 1. 首先构建所有包
npm run build

# 2. 发布到各平台
# 发布到 crates.io
cargo publish

# 发布到 npm
cd pkg-npm && npm publish

# 发布到 JSR
cd pkg-jsr && deno publish
```

## 许可证

本项目采用 MIT 许可证。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 更新日志

### 0.1.0
- 初始版本
- 基本的 DOCX 模板处理功能
- 支持多平台部署

## 测试

### 浏览器兼容性测试

本项目提供了完整的浏览器兼容性测试套件，确保 npm 包在各种浏览器环境中正常工作：

```bash
# 进入测试目录
cd tests/npm_test

# 启动测试服务器
node server.js

# 在浏览器中访问
# http://localhost:8080/tests/npm_test/browser_test_npm.html
```

测试功能包括：
- ✅ 包加载测试（多种构建版本）
- ✅ WASM 模块初始化
- ✅ 基础功能验证
- ✅ 实际文件处理测试
- ✅ 文件下载功能

支持的构建版本：
- `pkg-npm/` - npm 包，同时支持 Node.js 和浏览器环境
- `pkg-jsr/` - JSR 包，同时支持 Deno 和 Node.js 环境

### JSR 包测试

```bash
# JSR 包功能测试
cd tests/jsr_test
deno run --allow-net --allow-read --allow-write test.ts

# JSR 包综合测试
deno run --allow-net --allow-read --allow-write comprehensive_test.ts
```

### npm 包测试

```bash
# Node.js 环境测试
cd tests/npm_test
npm install
node test.js
```
