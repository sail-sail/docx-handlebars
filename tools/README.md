# Debug Tools / 调试工具

本目录包含用于调试和分析DOCX文件的工具脚本。

## 主要工具

### docx_debug.py ⭐ (推荐)
**统一的DOCX调试工具**，合并了原来多个调试脚本的功能，提供一个统一的调试接口。

**功能特性：**
- 🔍 基础检查：提取所有文本节点内容
- 📋 段落分析：按段落结构分析文本
- 🔬 详细结构：递归遍历XML结构
- 🔄 文件比较：比较两个DOCX文件的差异
- 🚀 完整分析：包含所有分析功能

**用法：**
```bash
# 基础检查（默认模式）
python docx_debug.py examples/template.docx

# 段落分析
python docx_debug.py -m paragraph examples/template.docx

# 详细结构分析
python docx_debug.py -m structure examples/template.docx

# 完整分析
python docx_debug.py -m full examples/template.docx

# 比较两个文件
python docx_debug.py -m compare examples/template.docx examples/output_deno.docx

# 分析多个文件
python docx_debug.py -m basic file1.docx file2.docx

# 不指定文件时，自动使用默认文件
python docx_debug.py
```

### 其他工具
- `build.js` - 构建脚本
- `update_version.js` - 版本更新脚本

## 依赖要求

调试工具只依赖Python标准库：
- `zipfile` - 处理DOCX文件（ZIP格式）
- `xml.etree.ElementTree` - 解析XML内容
- `argparse` - 命令行参数解析
- `pathlib` - 文件路径处理
- `re` - 正则表达式

## 使用说明

1. 确保有Python 3.6+环境
2. 在项目根目录运行脚本
3. 推荐使用新的 `docx_debug.py` 工具
