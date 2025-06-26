#!/usr/bin/env python3
import zipfile
import xml.etree.ElementTree as ET

def check_docx_content(file_path):
    try:
        with zipfile.ZipFile(file_path, 'r') as docx:
            with docx.open('word/document.xml') as doc_xml:
                content = doc_xml.read().decode('utf-8')
                
                # 提取所有文本节点内容
                root = ET.fromstring(content)
                texts = []
                for elem in root.iter():
                    if elem.tag.endswith('}t') and elem.text:
                        texts.append(elem.text)
                
                print(f'文件: {file_path}')
                print('文本内容:')
                for i, text in enumerate(texts):
                    print(f'{i+1:2d}. {repr(text)}')
                print()
                
                # 合并所有文本看看完整内容
                full_text = ''.join(texts)
                print('完整文本内容:')
                print(repr(full_text))
                print()
                print('可读文本内容:')
                print(full_text)
                
    except Exception as e:
        print(f'错误: {e}')

if __name__ == '__main__':
    print("检查模板文件...")
    check_docx_content('examples/template.docx')
    print("\n" + "="*50 + "\n")
    print("检查输出文件...")
    check_docx_content('examples/output_deno.docx')
