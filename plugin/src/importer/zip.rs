//! Just enough of the ZIP format to open compressed MusicXML (.mxl) archives.

use super::model::ImportResult;

pub struct Entry {
    pub name: String,
    method: u16,
    compressed_size: usize,
    local_offset: usize,
}

fn u16_at(bytes: &[u8], offset: usize) -> ImportResult<u16> {
    bytes
        .get(offset..offset + 2)
        .map(|slice| u16::from_le_bytes([slice[0], slice[1]]))
        .ok_or_else(|| "the .mxl archive is truncated".to_string())
}

fn u32_at(bytes: &[u8], offset: usize) -> ImportResult<u32> {
    bytes
        .get(offset..offset + 4)
        .map(|slice| u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
        .ok_or_else(|| "the .mxl archive is truncated".to_string())
}

/// List the archive's files from its central directory.
pub fn entries(bytes: &[u8]) -> ImportResult<Vec<Entry>> {
    const END_SIGNATURE: u32 = 0x0605_4b50;
    const ENTRY_SIGNATURE: u32 = 0x0201_4b50;
    let search_start = bytes.len().saturating_sub(22 + 65_535);
    let end = (search_start..bytes.len().saturating_sub(21))
        .rev()
        .find(|offset| u32_at(bytes, *offset).ok() == Some(END_SIGNATURE))
        .ok_or("the .mxl file is not a readable ZIP archive")?;
    let count = u16_at(bytes, end + 10)? as usize;
    let mut offset = u32_at(bytes, end + 16)? as usize;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        if u32_at(bytes, offset)? != ENTRY_SIGNATURE {
            return Err("the .mxl archive has a damaged file directory".into());
        }
        let method = u16_at(bytes, offset + 10)?;
        let compressed_size = u32_at(bytes, offset + 20)? as usize;
        let name_length = u16_at(bytes, offset + 28)? as usize;
        let extra_length = u16_at(bytes, offset + 30)? as usize;
        let comment_length = u16_at(bytes, offset + 32)? as usize;
        let local_offset = u32_at(bytes, offset + 42)? as usize;
        let name_bytes = bytes
            .get(offset + 46..offset + 46 + name_length)
            .ok_or("the .mxl archive is truncated")?;
        entries.push(Entry {
            name: String::from_utf8_lossy(name_bytes).into_owned(),
            method,
            compressed_size,
            local_offset,
        });
        offset += 46 + name_length + extra_length + comment_length;
    }
    Ok(entries)
}

/// Return one file's uncompressed contents.
pub fn read(bytes: &[u8], entry: &Entry) -> ImportResult<Vec<u8>> {
    let header = entry.local_offset;
    if u32_at(bytes, header)? != 0x0403_4b50 {
        return Err(format!("the .mxl archive entry {} is damaged", entry.name));
    }
    let start =
        header + 30 + u16_at(bytes, header + 26)? as usize + u16_at(bytes, header + 28)? as usize;
    let data = bytes
        .get(start..start + entry.compressed_size)
        .ok_or("the .mxl archive is truncated")?;
    match entry.method {
        0 => Ok(data.to_vec()),
        8 => miniz_oxide::inflate::decompress_to_vec(data).map_err(|_| {
            format!(
                "the .mxl archive entry {} could not be decompressed",
                entry.name
            )
        }),
        method => Err(format!(
            "the .mxl archive uses unsupported compression method {method}"
        )),
    }
}
