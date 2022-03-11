pub fn adjust_endianness_16(word: u16) -> u16 {
    u16::from_be_bytes(word.to_le_bytes())
}

pub fn adjust_endianness_32(dword: u32) -> u32 {
    u32::from_be_bytes(dword.to_le_bytes())
}

pub fn adjust_endianness_64(qword: u64) -> u64 {
    u64::from_be_bytes(qword.to_le_bytes())
}
