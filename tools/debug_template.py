#!/usr/bin/env python3
"""专门查看模板文件的段落结构"""

import zipfile
import xml.etree.ElementTree as ET
import sys

def analyze_template(file_path):
    """分析模板文件的段落结构"""
    print(f"=== 分析模板 {file_path} ===")
    
    with zipfile.ZipFile(file_path, 'r') as docx:
        # 读取 document.xml
        document_xml = docx.read('word/document.xml').decode('utf-8')
        
        # 解析XML
        root = ET.fromstring(document_xml)
        
        # 定义命名空间
        namespaces = {
            'w': 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'
        }
        
        # 找到所有段落
        paragraphs = root.findall('.//w:p', namespaces)
        print(f"总共有 {len(paragraphs)} 个段落:")
        
        for i, para in enumerate(paragraphs):
            print(f"  <p{i}> (段落 {i})")
            
            # 提取段落中的所有文本
            texts = para.findall('.//w:t', namespaces)
            paragraph_text = ""
            for text_elem in texts:
                if text_elem.text:
                    paragraph_text += text_elem.text
            
            print(f"    段落文本: '{paragraph_text}'")

if __name__ == "__main__":
    analyze_template("examples/template.docx")
