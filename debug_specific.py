import zipfile
import xml.etree.ElementTree as ET

def debug_docx_file(docx_path):
    print(f"=== 调试 {docx_path} ===")
    with zipfile.ZipFile(docx_path, 'r') as docx:
        # 读取 document.xml
        document_xml = docx.read('word/document.xml').decode('utf-8')
        
        # 解析 XML
        root = ET.fromstring(document_xml)
        
        # 定义命名空间
        namespaces = {
            'w': 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'
        }
        
        # 提取完整的可读文本
        full_text = ""
        
        # 遍历所有段落
        paragraphs = root.findall('.//w:p', namespaces)
        for i, paragraph in enumerate(paragraphs):
            print(f"  <p{i}> (段落 {i})")
            
            # 获取段落中的所有文本节点
            text_nodes = paragraph.findall('.//w:t', namespaces)
            paragraph_text = ""
            for j, text_node in enumerate(text_nodes):
                text_content = text_node.text or ""
                print(f"    <t{j}> (文本节点 {j})")
                print(f"      文本: '{text_content}'")
                paragraph_text += text_content
            
            print(f"    段落完整文本: '{paragraph_text}'")
            full_text += paragraph_text + "\n"
        
        print("==================================================")
        print("提取的完整文本:")
        print(repr(full_text))

if __name__ == "__main__":
    import sys
    
    files_to_debug = [
        "examples/output_rust.docx",
        "examples/output_deno.docx",  # 对比
    ]
    
    for file_path in files_to_debug:
        try:
            debug_docx_file(file_path)
            print()
        except Exception as e:
            print(f"处理 {file_path} 时出错: {e}")
            print()
