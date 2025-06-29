#!/usr/bin/env python3
"""
DOCX 调试工具
合并了原来的多个调试脚本功能，提供一个统一的调试接口
"""

import zipfile
import xml.etree.ElementTree as ET
import sys
import argparse
import re
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
    
    def extract_simple_text(self, root):
        """简单提取所有文本节点内容（原 check_template.py 逻辑）"""
        texts = []
        for elem in root.iter():
            if elem.tag.endswith('}t') and elem.text:
                texts.append(elem.text)
        return texts
    
    def extract_paragraphs(self, root):
        """按段落提取文本（原 debug_lines.py 逻辑）"""
        paragraphs = root.findall('.//w:p', self.namespaces)
        paragraph_texts = []
        
        for para in paragraphs:
            texts = para.findall('.//w:t', self.namespaces)
            paragraph_text = ""
            for text_elem in texts:
                if text_elem.text:
                    paragraph_text += text_elem.text
            paragraph_texts.append(paragraph_text)
        
        return paragraph_texts
    
    def walk_element_debug(self, elem, level=0):
        """递归遍历元素，显示详细结构（原 debug_extract.py 逻辑）"""
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
    
    def basic_check(self, file_path):
        """基础检查（原 check_template.py 功能）"""
        if not self.check_file_exists(file_path):
            return
        
        print(f"📄 基础检查: {file_path}")
        print("=" * 60)
        
        root = self.read_document_xml(file_path)
        if root is None:
            return
        
        # 提取所有文本节点
        texts = self.extract_simple_text(root)
        
        print(f"🔍 找到 {len(texts)} 个文本节点:")
        for i, text in enumerate(texts):
            print(f"  {i+1:2d}. {repr(text)}")
        
        # 合并文本
        full_text = ''.join(texts)
        print(f"\n📝 完整文本内容:")
        print(f"  原始: {repr(full_text)}")
        print(f"  可读: {full_text}")
        print()
    
    def paragraph_analysis(self, file_path):
        """段落分析（原 debug_lines.py 和 debug_template.py 功能）"""
        if not self.check_file_exists(file_path):
            return
        
        print(f"📋 段落分析: {file_path}")
        print("=" * 60)
        
        root = self.read_document_xml(file_path)
        if root is None:
            return
        
        paragraphs = self.extract_paragraphs(root)
        
        print(f"📑 总共 {len(paragraphs)} 个段落:")
        for i, para_text in enumerate(paragraphs):
            print(f"  段落 {i}: '{para_text}'")
            if not para_text.strip():
                print("    ⚠️  空段落")
        
        # 检查特定内容
        full_text = '\n'.join(paragraphs)
        print(f"\n🔍 查找特殊内容:")
        
        # 查找 Handlebars 模板语法
        handlebars_pattern = r'\{\{[^}]+\}\}'
        matches = re.findall(handlebars_pattern, full_text)
        if matches:
            print(f"  找到 {len(matches)} 个 Handlebars 模板:")
            for match in matches:
                print(f"    - {match}")
        else:
            print("  未找到 Handlebars 模板语法")
        
        # 查找特定关键词
        keywords = ["数据分析工具", "template", "{{"]
        for keyword in keywords:
            if keyword in full_text:
                lines = full_text.split('\n')
                for i, line in enumerate(lines):
                    if keyword in line:
                        print(f"  关键词 '{keyword}' 在第 {i} 行: '{line}'")
        print()
    
    def detailed_structure(self, file_path):
        """详细结构分析（原 debug_extract.py 和 debug_specific.py 功能）"""
        if not self.check_file_exists(file_path):
            return
        
        print(f"🔬 详细结构分析: {file_path}")
        print("=" * 60)
        
        root = self.read_document_xml(file_path)
        if root is None:
            return
        
        print("📊 XML 结构遍历:")
        extracted_text = self.walk_element_debug(root)
        
        print(f"\n📋 提取的完整文本:")
        print(f"  原始: {repr(extracted_text)}")
        print(f"  可读: {extracted_text}")
        print()
    
    def compare_files(self, file1, file2):
        """比较两个文件"""
        print(f"🔄 文件比较: {file1} vs {file2}")
        print("=" * 60)
        
        if not (self.check_file_exists(file1) and self.check_file_exists(file2)):
            return
        
        # 读取两个文件
        root1 = self.read_document_xml(file1)
        root2 = self.read_document_xml(file2)
        
        if root1 is None or root2 is None:
            return
        
        # 提取段落
        paragraphs1 = self.extract_paragraphs(root1)
        paragraphs2 = self.extract_paragraphs(root2)
        
        print(f"📊 段落数量比较:")
        print(f"  {file1}: {len(paragraphs1)} 个段落")
        print(f"  {file2}: {len(paragraphs2)} 个段落")
        
        print(f"\n📝 逐段落比较:")
        max_paragraphs = max(len(paragraphs1), len(paragraphs2))
        
        for i in range(max_paragraphs):
            p1 = paragraphs1[i] if i < len(paragraphs1) else "<缺失>"
            p2 = paragraphs2[i] if i < len(paragraphs2) else "<缺失>"
            
            if p1 == p2:
                print(f"  段落 {i}: ✅ 相同")
            else:
                print(f"  段落 {i}: ❌ 不同")
                print(f"    文件1: '{p1}'")
                print(f"    文件2: '{p2}'")
        print()
    
    def full_analysis(self, file_path):
        """完整分析（包含所有功能）"""
        print(f"🚀 完整分析: {file_path}")
        print("=" * 60)
        
        self.basic_check(file_path)
        self.paragraph_analysis(file_path)
        self.detailed_structure(file_path)


def main():
    parser = argparse.ArgumentParser(description='DOCX 文件调试工具')
    parser.add_argument('files', nargs='*', help='要分析的 DOCX 文件路径')
    parser.add_argument('-m', '--mode', choices=['basic', 'paragraph', 'structure', 'full', 'compare'], 
                       default='basic', help='分析模式')
    parser.add_argument('--compare', action='store_true', help='比较两个文件（需要提供两个文件路径）')
    
    args = parser.parse_args()
    
    debugger = DocxDebugger()
    
    # 如果没有提供文件，使用默认文件
    if not args.files:
        default_files = ['examples/template.docx', 'examples/output_deno.docx']
        print(f"📁 使用默认文件: {', '.join(default_files)}")
        args.files = [f for f in default_files if Path(f).exists()]
        
        if not args.files:
            print("❌ 未找到默认文件，请指定文件路径")
            return
    
    # 比较模式
    if args.compare or args.mode == 'compare':
        if len(args.files) < 2:
            print("❌ 比较模式需要至少两个文件")
            return
        debugger.compare_files(args.files[0], args.files[1])
        return
    
    # 单文件分析
    for file_path in args.files:
        if args.mode == 'basic':
            debugger.basic_check(file_path)
        elif args.mode == 'paragraph':
            debugger.paragraph_analysis(file_path)
        elif args.mode == 'structure':
            debugger.detailed_structure(file_path)
        elif args.mode == 'full':
            debugger.full_analysis(file_path)
        
        if len(args.files) > 1:
            print("\n" + "="*80 + "\n")


if __name__ == '__main__':
    main()
