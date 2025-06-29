#!/usr/bin/env python3
"""
测试表格保留功能的简单脚本
"""

import subprocess
import os

def test_table_preservation():
    print("🧪 测试表格保留功能")
    print("=" * 50)
    
    # 确保在正确的目录（项目根目录）
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.chdir(project_root)
    
    print("📁 运行Rust示例...")
    result = subprocess.run(["cargo", "run", "--example", "rust_example"], 
                           capture_output=True, text=True)
    
    if result.returncode == 0:
        print("✅ Rust示例运行成功")
        
        print("\n📊 检查输出文件结构...")
        # 直接检查输出文件内容
        import zipfile
        try:
            with zipfile.ZipFile("examples/output_rust.docx", 'r') as z:
                content = z.read('word/document.xml').decode('utf-8')
                
                # 检查是否包含表格结构（Word使用w:命名空间，且可能有属性）
                has_table = "<w:tbl" in content
                has_table_row = "<w:tr" in content  
                has_table_cell = "<w:tc" in content
            
            print(f"🔍 表格检查结果:")
            print(f"   - 包含表格 (<tbl>): {'✅' if has_table else '❌'}")
            print(f"   - 包含表格行 (<tr>): {'✅' if has_table_row else '❌'}")
            print(f"   - 包含表格单元格 (<tc>): {'✅' if has_table_cell else '❌'}")
            
            if has_table and has_table_row and has_table_cell:
                print("\n🎉 测试通过！表格结构已成功保留")
                
                # 计算表格统计（匹配带属性的标签）
                import re
                table_count = len(re.findall(r'<w:tbl[^>]*>', content))
                row_count = len(re.findall(r'<w:tr[^>]*>', content))
                cell_count = len(re.findall(r'<w:tc[^>]*>', content))
                
                print(f"\n📈 表格统计:")
                print(f"   - 表格数量: {table_count}")
                print(f"   - 行数量: {row_count}")  
                print(f"   - 单元格数量: {cell_count}")
                
            else:
                print("\n❌ 测试失败！表格结构未正确保留")
                
        except Exception as e:
            print(f"❌ 检查文件时出错: {e}")
            
    else:
        print(f"❌ Rust示例运行失败: {result.stderr}")

if __name__ == "__main__":
    test_table_preservation()
