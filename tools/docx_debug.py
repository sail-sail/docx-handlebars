#!/usr/bin/env python3
"""
DOCX 调试工具
递归遍历 DOCX 文件的 XML 结构，显示详细的元素和文本内容
"""

import zipfile
import xml.etree.ElementTree as ET
import sys
import argparse
from pathlib import Path


class DocxDebugger:
    """DOCX 文件调试器"""
    
    def __init__(self):
        self.namespaces = {
            'w': 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'
        }
    
    def check_file_exists(self, file_path):
        """检查文件是否存在"""
        if not Path(file_path).exists():
            print(f"❌ 文件不存在: {file_path}")
            return False
        return True
    
    def read_document_xml(self, file_path):
        """读取 DOCX 文件中的 document.xml"""
        try:
            with zipfile.ZipFile(file_path, 'r') as docx:
                with docx.open('word/document.xml') as doc_xml:
                    content = doc_xml.read().decode('utf-8')
                    return ET.fromstring(content)
        except Exception as e:
            print(f"❌ 读取文件失败: {e}")
            return None
    
    
    def walk_element_debug(self, elem, level=0):
        """递归遍历元素，显示详细结构"""
        tag_name = elem.tag.split('}')[-1] if '}' in elem.tag else elem.tag
        indent = "  " * level
        
        text_content = ""
        
        if tag_name == 't':  # w:t
            print(f"{indent}<{tag_name}> (文本元素)")
            if elem.text:
                print(f"{indent}  文本: {repr(elem.text)}")
                text_content += elem.text
        elif tag_name == 'p':  # w:p
            print(f"{indent}<{tag_name}> (段落)")
        elif tag_name == 'br':  # w:br
            print(f"{indent}<{tag_name}> (换行)")
            text_content += '\n'
        else:
            print(f"{indent}<{tag_name}>")
        
        # 递归处理子元素
        for child in elem:
            child_text = self.walk_element_debug(child, level + 1)
            text_content += child_text
        
        return text_content
    
    def analyze_docx(self, file_path):
        """分析 DOCX 文件结构"""
        if not self.check_file_exists(file_path):
            return
        
        print(f"🔬 分析 DOCX 文件: {file_path}")
        print("=" * 60)
        
        root = self.read_document_xml(file_path)
        if root is None:
            return
        
        print("📊 XML 结构遍历:")
        extracted_text = self.walk_element_debug(root)
        
        print(f"\n📋 提取的完整文本:")
        print(f"  原始: {repr(extracted_text)}")
        print(f"  可读文本:")
        print(extracted_text)
        print()


def main():
    parser = argparse.ArgumentParser(description='DOCX 文件调试工具 - 递归遍历XML结构')
    parser.add_argument('files', nargs='*', help='要分析的 DOCX 文件路径')
    
    args = parser.parse_args()
    
    debugger = DocxDebugger()
    
    # 如果没有提供文件，使用默认文件
    if not args.files:
        default_files = ['examples/template.docx', 'examples/output_deno.docx']
        print(f"📁 使用默认文件: {', '.join(default_files)}")
        args.files = [f for f in default_files if Path(f).exists()]
        
        if not args.files:
            print("❌ 未找到默认文件，请指定文件路径")
            print("用法: python docx_debug.py <docx文件路径>")
            return
    
    # 分析文件
    for file_path in args.files:
        debugger.analyze_docx(file_path)
        
        if len(args.files) > 1:
            print("\n" + "="*80 + "\n")


if __name__ == '__main__':
    main()
