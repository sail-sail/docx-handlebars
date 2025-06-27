# Tests / 测试目录

本目录包含 docx-handlebars 项目的各种测试套件。

## 📋 测试目录结构

```
tests/
├── integration_tests.rs  # Rust 集成测试
├── wasm_tests.rs         # WASM 功能测试
├── jsr_test/             # JSR 包测试套件
└── npm_test/             # npm 包测试套件
```

## 🧪 测试套件说明

### Rust 集成测试
- `integration_tests.rs` - Rust 库的集成测试
- `wasm_tests.rs` - WebAssembly 绑定测试

### JSR 包测试 (`jsr_test/`)
针对 [JSR (JavaScript Registry)](https://jsr.io/@sail/docx-handlebars) 发布包的测试：

```bash
cd tests/jsr_test

# 综合功能测试
deno run --allow-net --allow-read --allow-write test.ts
```

**测试内容：**
- Deno 环境兼容性
- API 功能验证
- 文件读写操作
- 错误处理

### npm 包测试 (`npm_test/`)
针对 [npm](https://www.npmjs.com/package/docx-handlebars) 发布包的测试：

```bash
cd tests/npm_test

# Node.js 环境测试
npm install
node test.mjs

# 浏览器兼容性测试
node server.js
# 然后访问: http://localhost:8080/tests/npm_test/browser_test_npm.html
```

**测试内容：**
- Node.js 环境兼容性
- 浏览器环境兼容性
- WASM 模块加载
- 文件处理功能
- 多种导入方式验证

## 🚀 运行所有测试

```bash
# Rust 测试
cargo test

# WASM 测试
wasm-pack test --headless --firefox

# JSR 测试
cd tests/jsr_test && deno test --allow-net --allow-read --allow-write

# npm 测试
cd tests/npm_test && npm test
```

## 📊 测试报告

各测试套件都会生成相应的输出文件和报告：

- **JSR 测试**: `tests/jsr_test/output_jsr_test.docx`
- **npm 测试**: `tests/npm_test/test_output_npm.docx`
- **浏览器测试**: 在线交互式测试界面

## 🔍 测试策略

1. **单元测试**: Rust 代码的单元测试
2. **集成测试**: 跨模块功能测试
3. **平台测试**: 各发布平台的兼容性测试
4. **浏览器测试**: Web 环境的兼容性测试
5. **回归测试**: 确保修复不破坏现有功能

---

*更多测试相关信息请参考各测试目录下的具体文档。*
