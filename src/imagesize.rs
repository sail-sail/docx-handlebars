

/// 获取 PNG 图片的宽高
fn get_png_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 24 || &data[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    // IHDR chunk: starts at byte 8, width at 16~19, height at 20~23
    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Some((width, height))
}

/// 获取 JPEG 图片的宽高
fn get_jpeg_dimensions(data: &[u8]) -> Option<(u16, u16)> {
    let mut i = 2;
    while i + 9 < data.len() {
        if data[i] != 0xFF {
            i += 1;
            continue;
        }
        let marker = data[i + 1];
        let len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
        if marker == 0xC0 || marker == 0xC2 {
            // SOF0 or SOF2
            let height = u16::from_be_bytes([data[i + 5], data[i + 6]]);
            let width = u16::from_be_bytes([data[i + 7], data[i + 8]]);
            return Some((width, height));
        }
        i += 2 + len;
    }
    None
}

/// 获取 WebP 图片的宽高
fn get_webp_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 30 || &data[0..4] != b"RIFF" || &data[8..12] != b"WEBP" {
        return None;
    }
    // VP8X
    if &data[12..16] == b"VP8X" {
        let width = 1 + u32::from_le_bytes([data[24], data[25], data[26], 0]);
        let height = 1 + u32::from_le_bytes([data[27], data[28], data[29], 0]);
        return Some((width, height));
    }
    // VP8
    if &data[12..15] == b"VP8" && data[15] == b' ' {
        // Lossy
        let width = u16::from_le_bytes([data[26], data[27]]) as u32;
        let height = u16::from_le_bytes([data[28], data[29]]) as u32;
        return Some((width, height));
    }
    // VP8L
    if &data[12..16] == b"VP8L" {
        let b = &data[21..25];
        let width = 1 + (((b[1] & 0x3F) as u32) << 8 | b[0] as u32);
        let height = 1 + (((b[3] & 0xF) as u32) << 10 | (b[2] as u32) << 2 | ((b[1] & 0xC0) as u32) >> 6);
        return Some((width, height));
    }
    None
}

/// 获取 BMP 图片的宽高
fn get_bmp_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 26 || &data[0..2] != b"BM" {
        return None;
    }
    // BMP 文件头后是 DIB 头，宽高在偏移 18~21（宽），22~25（高），都是 little-endian
    let width = u32::from_le_bytes([data[18], data[19], data[20], data[21]]);
    let height = u32::from_le_bytes([data[22], data[23], data[24], data[25]]);
    Some((width, height))
}

fn get_tiff_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 8 {
        return None;
    }
    // 判断字节序
    let le = &data[0..2] == b"II";
    let be = &data[0..2] == b"MM";
    if !le && !be {
        return None;
    }
    let read_u16 = |d: &[u8]| if le {
        u16::from_le_bytes([d[0], d[1]])
    } else {
        u16::from_be_bytes([d[0], d[1]])
    };
    let read_u32 = |d: &[u8]| if le {
        u32::from_le_bytes([d[0], d[1], d[2], d[3]])
    } else {
        u32::from_be_bytes([d[0], d[1], d[2], d[3]])
    };
    // 检查 magic number
    let magic = read_u16(&data[2..4]);
    if magic != 42 {
        return None;
    }
    // IFD 偏移
    let ifd_offset = read_u32(&data[4..8]) as usize;
    if data.len() < ifd_offset + 2 {
        return None;
    }
    let num_dir = read_u16(&data[ifd_offset..ifd_offset + 2]) as usize;
    let mut width = None;
    let mut height = None;
    for i in 0..num_dir {
        let entry = ifd_offset + 2 + i * 12;
        if data.len() < entry + 12 {
            break;
        }
        let tag = read_u16(&data[entry..entry + 2]);
        let field_type = read_u16(&data[entry + 2..entry + 4]);
        // let count = read_u32(&data[entry + 4..entry + 8]);
        let value_offset = &data[entry + 8..entry + 12];
        // tag 256: ImageWidth, tag 257: ImageLength
        if tag == 256 {
            width = Some(match field_type {
                3 => read_u16(value_offset) as u32, // SHORT
                4 => read_u32(value_offset),        // LONG
                _ => continue,
            });
        }
        if tag == 257 {
            height = Some(match field_type {
                3 => read_u16(value_offset) as u32,
                4 => read_u32(value_offset),
                _ => continue,
            });
        }
        if width.is_some() && height.is_some() {
            break;
        }
    }
    match (width, height) {
        (Some(w), Some(h)) => Some((w, h)),
        _ => None,
    }
}

fn get_gif_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 10 || (&data[0..6] != b"GIF87a" && &data[0..6] != b"GIF89a") {
        return None;
    }
    let width = u16::from_le_bytes([data[6], data[7]]) as u32;
    let height = u16::from_le_bytes([data[8], data[9]]) as u32;
    Some((width, height))
}

// 获取图片的宽高
/// 支持 PNG, JPEG, WebP, BMP, TIFF, GIF 格式
pub fn get_image_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if let Some((w, h)) = get_png_dimensions(data) {
        Some((w, h))
    } else if let Some((w, h)) = get_jpeg_dimensions(data).map(|(w, h)| (w as u32, h as u32)) {
        Some((w, h))
    } else if let Some((w, h)) = get_webp_dimensions(data) {
        Some((w, h))
    } else if let Some((w, h)) = get_bmp_dimensions(data) {
        Some((w, h))
    } else if let Some((w, h)) = get_tiff_dimensions(data) {
        Some((w, h))
    } else if let Some((w, h)) = get_gif_dimensions(data) {
        Some((w, h))
    } else {
        None
    }
}
