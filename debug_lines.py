#!/usr/bin/env python3
"""调试渲染后文本的行分布"""

import zipfile
import xml.etree.ElementTree as ET
import sys
import re

def extract_text_lines(xml_content):
    """提取文本，保留换行信息"""
    root = ET.fromstring(xml_content)
    
    # 定义命名空间
    namespaces = {
        'w': 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'
    }
    
    text_content = ""
    
    # 找到所有段落
    paragraphs = root.findall('.//w:p', namespaces)
    for i, para in enumerate(paragraphs):
        if i > 0:  # 不是第一个段落则添加换行
            text_content += "\n"
        
        # 提取段落中的所有文本
        texts = para.findall('.//w:t', namespaces)
        for text_elem in texts:
            if text_elem.text:
                text_content += text_elem.text
    
    return text_content

def analyze_docx(file_path):
    """分析 DOCX 文件"""
    print(f"=== 分析 {file_path} ===")
    
    with zipfile.ZipFile(file_path, 'r') as docx:
        # 读取 document.xml
        document_xml = docx.read('word/document.xml').decode('utf-8')
        
        # 提取文本内容
        text_content = extract_text_lines(document_xml)
        
        print("提取的完整文本:")
        lines = text_content.split('\n')
        for i, line in enumerate(lines):
            print(f"Line {i}: '{line}'")
        
        print(f"\n总共 {len(lines)} 行")
        
        # 查找数据分析工具相关的行
        for i, line in enumerate(lines):
            if "数据分析工具" in line:
                print(f"\n找到'数据分析工具'在第 {i} 行")
                print(f"当前行: '{line}'")
                if i + 1 < len(lines):
                    print(f"下一行: '{lines[i + 1]}'")
                    if lines[i + 1].strip() == "":
                        print("⚠️  下一行是空行！")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("使用方法: python debug_lines.py <docx文件路径>")
        sys.exit(1)
    
    analyze_docx(sys.argv[1])
