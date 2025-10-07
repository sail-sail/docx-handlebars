use std::io::{Cursor, Write};

#[allow(dead_code)]
/// 根据真实Word OLE文件结构创建标准的OLE复合文档对象
pub(crate) fn create_ole_object(
    attachment_bytes: &[u8],
    filename: Option<&str>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut ole_data = vec![0u8; 3072]; // 6个扇区，每个512字节
    let mut cursor = Cursor::new(&mut ole_data);
    
    let filename_str = filename.unwrap_or("file.bin");
    
    // 写入CFB Header (扇区0)
    write_cfb_header(&mut cursor)?;
    
    // 写入FAT Table (扇区1) 
    cursor.set_position(512);
    write_fat_table(&mut cursor)?;
    
    // 写入Directory Entries (扇区2)
    cursor.set_position(1024);
    write_directory_entries(&mut cursor, attachment_bytes.len(), filename_str)?;
    
    // 写入Mini FAT (扇区3)
    cursor.set_position(1536);
    write_mini_fat(&mut cursor)?;
    
    // 写入迷你流数据 (扇区4和5)
    cursor.set_position(2048);
    write_ministream_data(&mut cursor, attachment_bytes, filename_str)?;
    
    Ok(ole_data)
}

/// 写入CFB文件头部（按真实Word文件格式）
fn write_cfb_header(cursor: &mut Cursor<&mut Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    // CFB签名 - 参考 common/constants.rs MAGIC_BYTES
    cursor.write_all(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1])?;
    
    // CLSID (16字节零) - 必须为全零
    cursor.write_all(&[0; 16])?;
    
    // 次版本号 (0x003E) - 参考 common 中的标准
    cursor.write_all(&0x003E_u16.to_le_bytes())?;
    
    // 主版本号 (0x0003) - Version 3，512字节扇区大小
    cursor.write_all(&0x0003_u16.to_le_bytes())?;
    
    // 字节序标识 (0xFFFE) - Little Endian标识
    cursor.write_all(&0xFFFE_u16.to_le_bytes())?;
    
    // 扇区大小指数 (0x0009 = 512字节) - 2^9 = 512
    cursor.write_all(&0x0009_u16.to_le_bytes())?;
    
    // 迷你扇区大小指数 (0x0006 = 64字节) - 2^6 = 64
    cursor.write_all(&0x0006_u16.to_le_bytes())?;
    
    // 保留字段 (6字节零)
    cursor.write_all(&[0; 6])?;
    
    // 目录扇区总数 (0) - Version 3 必须为 0
    cursor.write_all(&0_u32.to_le_bytes())?;
    
    // FAT扇区数 (1)
    cursor.write_all(&1_u32.to_le_bytes())?;
    
    // 第一个目录扇区 (扇区2) - 修正为正确的目录扇区位置
    cursor.write_all(&2_u32.to_le_bytes())?;
    
    // 事务签名 (0)
    cursor.write_all(&0_u32.to_le_bytes())?;
    
    // 迷你流截止阈值 (4096) - 标准值
    cursor.write_all(&4096_u32.to_le_bytes())?;
    
    // 第一个迷你FAT扇区 (扇区3) - 迷你FAT位置
    cursor.write_all(&3_u32.to_le_bytes())?;
    
    // 迷你FAT扇区数 (1)
    cursor.write_all(&1_u32.to_le_bytes())?;
    
    // 第一个DIFAT扇区 (ENDOFCHAIN) - 没有额外的DIFAT扇区
    cursor.write_all(&0xFFFFFFFE_u32.to_le_bytes())?;
    
    // DIFAT扇区数 (0)
    cursor.write_all(&0_u32.to_le_bytes())?;
    
    // DIFAT数组的前109个条目 (只有第一个是1，其余为FREESECT)
    cursor.write_all(&1_u32.to_le_bytes())?; // FAT在扇区1
    for _ in 1..109 {
        cursor.write_all(&0xFFFFFFFF_u32.to_le_bytes())?; // UNALLOCATED_SECTOR
    }
    
    Ok(())
}

/// 写入FAT表（按真实Word文件格式）
fn write_fat_table(cursor: &mut Cursor<&mut Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    // 扇区0: FATSECT (CFB头部)
    cursor.write_all(&0xFFFFFFFD_u32.to_le_bytes())?;
    
    // 扇区1: ENDOFCHAIN (FAT表结束)
    cursor.write_all(&0xFFFFFFFE_u32.to_le_bytes())?;
    
    // 扇区2: ENDOFCHAIN (目录扇区结束)
    cursor.write_all(&0xFFFFFFFE_u32.to_le_bytes())?;
    
    // 扇区3: ENDOFCHAIN (Mini FAT扇区结束)
    cursor.write_all(&0xFFFFFFFE_u32.to_le_bytes())?;
    
    // 扇区4: Next=5 (迷你流数据链接到下一个扇区)
    cursor.write_all(&5_u32.to_le_bytes())?;
    
    // 扇区5: ENDOFCHAIN (迷你流数据结束)
    cursor.write_all(&0xFFFFFFFE_u32.to_le_bytes())?;
    
    // 填充剩余的FAT条目为UNALLOCATED_SECTOR (0xFFFFFFFF)
    for _ in 6..128 {
        cursor.write_all(&0xFFFFFFFF_u32.to_le_bytes())?;
    }
    
    Ok(())
}

/// 写入迷你流数据（扇区4和5）
fn write_ministream_data(
    cursor: &mut Cursor<&mut Vec<u8>>,
    attachment_bytes: &[u8],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 迷你扇区0: ObjInfo数据 (6字节)
    cursor.write_all(&[0x00, 0x00, 0x03, 0x00, 0x0D, 0x00])?;
    // 填充到64字节
    cursor.write_all(&[0; 58])?;
    
    // 迷你扇区1开始: Ole10Native数据
    write_ole10_native_data(cursor, attachment_bytes, filename)?;
    
    Ok(())
}

/// 写入Ole10Native数据到迷你流
fn write_ole10_native_data(
    cursor: &mut Cursor<&mut Vec<u8>>,
    attachment_bytes: &[u8],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ole10Native格式:
    // 4字节: 嵌入数据大小
    cursor.write_all(&(attachment_bytes.len() as u32).to_le_bytes())?;
    
    // 嵌入数据
    cursor.write_all(attachment_bytes)?;
    
    // 标签字符串 (null-terminated)
    let label = format!("{filename} Document");
    cursor.write_all(label.as_bytes())?;
    cursor.write_all(&[0])?; // null terminator
    
    // 文件名字符串 (null-terminated)
    cursor.write_all(filename.as_bytes())?;
    cursor.write_all(&[0])?; // null terminator
    
    // 文件路径字符串 (null-terminated，通常为空)
    cursor.write_all(&[0])?; // empty path + null terminator
    
    // 类型标识符 (4字节，通常为0)
    cursor.write_all(&[0; 4])?;
    
    // 临时路径长度 (4字节，通常为0)
    cursor.write_all(&[0; 4])?;
    
    Ok(())
}

/// 写入Mini FAT（在扇区3的开头512字节）
fn write_mini_fat(cursor: &mut Cursor<&mut Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    // 根据真实Word文件的Mini FAT结构
    // Mini-sector 0: ENDOFCHAIN (ObjInfo数据)
    cursor.write_all(&0xFFFFFFFE_u32.to_le_bytes())?;
    
    // Mini-sector 1: Next=2 (Ole10Native数据开始)
    cursor.write_all(&2_u32.to_le_bytes())?;
    
    // Mini-sector 2: Next=3 (Ole10Native数据继续)
    cursor.write_all(&3_u32.to_le_bytes())?;
    
    // Mini-sector 3: Next=4 (Ole10Native数据继续)
    cursor.write_all(&4_u32.to_le_bytes())?;
    
    // Mini-sector 4: ENDOFCHAIN (Ole10Native数据结束)
    cursor.write_all(&0xFFFFFFFE_u32.to_le_bytes())?;
    
    // 其余的mini-sectors设为FREESECT
    for _ in 5..128 {
        cursor.write_all(&0xFFFFFFFC_u32.to_le_bytes())?;
    }
    Ok(())
}

/// 写入目录条目（按真实Word文件格式）
fn write_directory_entries(
    cursor: &mut Cursor<&mut Vec<u8>>,
    data_len: usize,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Root Entry (128字节)
    write_root_entry(cursor)?;
    
    // ObjInfo Entry (128字节)  
    write_objinfo_entry(cursor)?;
    
    // Ole10Native Entry (128字节)
    write_ole10native_entry(cursor, data_len, filename)?;
    
    // 空的第四个条目 (128字节)
    cursor.write_all(&[0; 128])?;
    
    Ok(())
}

/// 写入Root目录条目
fn write_root_entry(cursor: &mut Cursor<&mut Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    // 名称: "Root Entry" (UTF-16LE) - 每个字符2字节
    let name = "Root Entry";
    let mut name_utf16 = Vec::new();
    for ch in name.chars() {
        name_utf16.extend_from_slice(&(ch as u16).to_le_bytes());
    }
    name_utf16.resize(64, 0); // 填充到64字节
    cursor.write_all(&name_utf16)?;
    
    // 名称长度 (22字节，包含null终止符)
    cursor.write_all(&22_u16.to_le_bytes())?;
    
    // 条目类型: Root Storage (5)
    cursor.write_all(&[5])?;
    
    // 颜色: Red (0)
    cursor.write_all(&[0])?;
    
    // 左兄弟: None (NOSTREAM)
    cursor.write_all(&0xFFFFFFFF_u32.to_le_bytes())?;
    
    // 右兄弟: None (NOSTREAM)
    cursor.write_all(&0xFFFFFFFF_u32.to_le_bytes())?;
    
    // 子节点: 指向条目1 (ObjInfo)
    cursor.write_all(&1_u32.to_le_bytes())?;
    
    // CLSID (16字节零)
    cursor.write_all(&[0; 16])?;
    
    // 状态位 (4字节零)
    cursor.write_all(&[0; 4])?;
    
    // 创建时间 (8字节零)
    cursor.write_all(&[0; 8])?;
    
    // 修改时间 (8字节零)
    cursor.write_all(&[0; 8])?;
    
    // 起始扇区: 扇区4 (Root的迷你流起始位置)
    cursor.write_all(&4_u32.to_le_bytes())?;
    
    // 大小: 迷你流的总大小 (按64字节对齐)
    let ministream_size = 64 * 8; // 8个迷你扇区 * 64字节
    cursor.write_all(&(ministream_size as u64).to_le_bytes())?;
    
    Ok(())
}

/// 写入ObjInfo目录条目
fn write_objinfo_entry(cursor: &mut Cursor<&mut Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    // 名称: "ObjInfo" (UTF-16LE)
    let name = "ObjInfo";
    let mut name_utf16 = Vec::new();
    for ch in name.chars() {
        name_utf16.extend_from_slice(&(ch as u16).to_le_bytes());
    }
    name_utf16.resize(64, 0); // 填充到64字节
    cursor.write_all(&name_utf16)?;
    
    // 名称长度 (18字节，包含null终止符) - 匹配真实文件
    cursor.write_all(&18_u16.to_le_bytes())?;
    
    // 条目类型: Stream (2)
    cursor.write_all(&[2])?;
    
    // 颜色: Black (1)
    cursor.write_all(&[1])?;
    
    // 左兄弟: None
    cursor.write_all(&0xFFFFFFFF_u32.to_le_bytes())?;
    
    // 右兄弟: 指向条目2 (Ole10Native)
    cursor.write_all(&2_u32.to_le_bytes())?;
    
    // 子节点: None
    cursor.write_all(&0xFFFFFFFF_u32.to_le_bytes())?;
    
    // CLSID (16字节零)
    cursor.write_all(&[0; 16])?;
    
    // 状态位 (4字节零)
    cursor.write_all(&[0; 4])?;
    
    // 创建时间 (8字节零)
    cursor.write_all(&[0; 8])?;
    
    // 修改时间 (8字节零)
    cursor.write_all(&[0; 8])?;
    
    // 起始扇区: 迷你扇区0 (ObjInfo在迷你流中)
    cursor.write_all(&0_u32.to_le_bytes())?;
    
    // 大小: 6字节
    cursor.write_all(&6_u64.to_le_bytes())?;
    
    Ok(())
}

/// 写入Ole10Native目录条目
fn write_ole10native_entry(
    cursor: &mut Cursor<&mut Vec<u8>>,
    data_len: usize,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 名称: "\x01Ole10Native" (UTF-16LE) - 带有特殊前缀
    let mut name_utf16 = Vec::new();
    // 前缀字节 0x01
    name_utf16.extend_from_slice(&[0x01, 0x00]);
    let name = "Ole10Native";
    for ch in name.chars() {
        name_utf16.extend_from_slice(&(ch as u16).to_le_bytes());
    }
    // null终止符
    name_utf16.extend_from_slice(&[0x00, 0x00]);
    name_utf16.resize(64, 0); // 填充到64字节
    cursor.write_all(&name_utf16)?;
    
    // 名称长度 (26字节，包含前缀和null终止符)
    cursor.write_all(&26_u16.to_le_bytes())?;
    
    // 条目类型: Stream (2)
    cursor.write_all(&[2])?;
    
    // 颜色: Red (0)
    cursor.write_all(&[0])?;
    
    // 左兄弟: None (NOSTREAM)
    cursor.write_all(&0xFFFFFFFF_u32.to_le_bytes())?;
    
    // 右兄弟: None (NOSTREAM)
    cursor.write_all(&0xFFFFFFFF_u32.to_le_bytes())?;
    
    // 子节点: None (NOSTREAM)
    cursor.write_all(&0xFFFFFFFF_u32.to_le_bytes())?;
    
    // CLSID (16字节零)
    cursor.write_all(&[0; 16])?;
    
    // 状态位 (4字节零)
    cursor.write_all(&[0; 4])?;
    
    // 创建时间 (8字节零)
    cursor.write_all(&[0; 8])?;
    
    // 修改时间 (8字节零)
    cursor.write_all(&[0; 8])?;
    
    // 起始扇区: 迷你扇区1 (Ole10Native在迷你流中的起始位置)
    cursor.write_all(&1_u32.to_le_bytes())?;
    
    // 计算Ole10Native数据的实际大小
    let ole10_size = calculate_ole10_size(data_len, filename);
    cursor.write_all(&(ole10_size as u64).to_le_bytes())?;
    
    Ok(())
}

/// 计算Ole10Native流的准确大小
fn calculate_ole10_size(data_len: usize, filename: &str) -> usize {
    // 4字节数据大小 + 数据 + 标签 + null + 文件名 + null + 路径null + 类型(4字节) + 临时路径长度(4字节)
    let label = format!("{filename} Document");
    4 + data_len + label.len() + 1 + filename.len() + 1 + 1 + 4 + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ole_object() {
        let test_data = b"Test OLE embedding content";
        let filename = "test.txt";
        
        let result = create_ole_object(test_data, Some(filename));
        assert!(result.is_ok());
        
        let ole_data = result.unwrap();
        assert_eq!(ole_data.len(), 3072); // 6个扇区
        
        // 验证CFB签名
        assert_eq!(&ole_data[0..8], &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]);
        
        // 验证主版本号
        let major_version = u16::from_le_bytes([ole_data[26], ole_data[27]]);
        assert_eq!(major_version, 0x0003);
        
        // 保存测试文件
        std::fs::write("oleObject1.bin", &ole_data).unwrap();
        println!("生成的OLE文件已保存为: oleObject1.bin");
    }
}
