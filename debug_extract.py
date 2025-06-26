#!/usr/bin/env python3
import zipfile
import xml.etree.ElementTree as ET

def extract_text_debug(file_path):
    try:
        with zipfile.ZipFile(file_path, 'r') as docx:
            with docx.open('word/document.xml') as doc_xml:
                content = doc_xml.read().decode('utf-8')
                
                print(f"=== 调试 {file_path} ===")
                
                # 提取完整的 XML 结构
                root = ET.fromstring(content)
                
                # 模拟 Rust 代码的逻辑
                text_content = ""
                in_text = False
                
                def walk_element(elem, level=0):
                    nonlocal text_content, in_text
                    
                    tag_name = elem.tag.split('}')[-1] if '}' in elem.tag else elem.tag
                    indent = "  " * level
                    
                    if tag_name == 't':  # w:t
                        in_text = True
                        print(f"{indent}<{tag_name}> (text element)")
                        if elem.text:
                            print(f"{indent}  文本: {repr(elem.text)}")
                            text_content += elem.text
                        in_text = False
                    elif tag_name == 'p':  # w:p
                        print(f"{indent}<{tag_name}> (paragraph)")
                        if text_content and not text_content.endswith('\n'):
                            # text_content += '\n'  # 注释掉，看看是否影响
                            pass
                    elif tag_name == 'br':  # w:br
                        print(f"{indent}<{tag_name}> (line break)")
                        text_content += '\n'
                    else:
                        print(f"{indent}<{tag_name}>")
                    
                    # 递归处理子元素
                    for child in elem:
                        walk_element(child, level + 1)
                
                walk_element(root)
                
                print(f"\n提取的完整文本:")
                print(repr(text_content))
                print(f"\n可读文本:")
                print(text_content)
                
                return text_content
                
    except Exception as e:
        print(f'错误: {e}')
        return None

if __name__ == '__main__':
    extract_text_debug('examples/template.docx')
    print("\n" + "="*50 + "\n")
    extract_text_debug('examples/output_deno.docx')
