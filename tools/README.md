# Debug Tools / 调试工具

本目录包含用于调试和分析DOCX文件的工具脚本。

## 主要工具

### docx_debug.py ⭐ (推荐)
**简洁的DOCX调试工具**，专注于递归遍历 DOCX 文件的 XML 结构，显示详细的元素和文本内容。

**功能特性：**
- � 递归遍历：深度遍历 XML 结构，显示每个元素的层级关系
- 📝 文本提取：提取并显示所有文本内容
- 🏷️ 元素标识：清楚标识段落、文本元素、换行等
- � 结构可视化：通过缩进显示 XML 层级结构

**用法：**
```bash
# 分析单个文件
python docx_debug.py examples/template.docx

# 分析多个文件
python docx_debug.py file1.docx file2.docx

# 不指定文件时，自动使用默认文件
python docx_debug.py
```

### test_table.py 🧪
**表格保留功能测试脚本**，验证DOCX模板处理过程中表格结构是否被正确保留。

**功能特性：**
- 🏃 自动运行Rust示例
- 📊 检查输出文件的表格结构
- 📈 统计表格数量、行数、单元格数量
- ✅ 提供详细的测试结果报告

**用法：**
```bash
# 运行表格保留功能测试
python test_table.py
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

## 使用说明

1. 确保有Python 3.6+环境
2. 在项目根目录运行脚本
3. 推荐使用新的 `docx_debug.py` 工具
