# Debug Tools / 调试工具

本目录包含用于调试和分析DOCX文件的Python工具脚本。

## 工具说明

### check_template.py
检查DOCX文件的内容，提取并显示所有文本节点。

**用法：**
```bash
python check_template.py
```

### debug_extract.py
调试文本提取过程，模拟Rust代码的逻辑来分析DOCX文件结构。

**用法：**
```bash
python debug_extract.py
```

### debug_lines.py
调试渲染后文本的行分布，保留换行信息进行分析。

**用法：**
```bash
python debug_lines.py [docx_file_path]
```

### debug_specific.py
针对特定DOCX文件进行调试，提取完整的可读文本。

**用法：**
```bash
python debug_specific.py
```

### debug_template.py
专门分析模板文件的段落结构。

**用法：**
```bash
python debug_template.py [template_file_path]
```

## 依赖要求

这些脚本都是纯Python脚本，只依赖标准库：
- `zipfile` - 处理DOCX文件（实际是ZIP格式）
- `xml.etree.ElementTree` - 解析XML内容
- `sys` - 系统操作
- `re` - 正则表达式

## 使用说明

1. 确保你有Python 3.x环境
2. 在项目根目录或tools目录下运行相应的脚本
3. 某些脚本可能需要修改脚本内的文件路径
