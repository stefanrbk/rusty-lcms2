use std::io::*;
use std::convert::TryInto;
use std::mem::size_of;

use crate::*;

mod vec3;

mod mat3;

/// ICC base tag
mod tag_base;

#[cfg(test)]
mod tests;

// Public exports
pub use mat3::Mat3;
pub use tag_base::TagBase;
pub use vec3::Vec3;

// const READ_ADJUST_ENDIANNESS_U32: &dyn Fn([u8; 4]) -> u32 = if CMS_USE_BIG_ENDIAN {&u32::from_be_bytes} else {&u32::from_le_bytes};
// const WRITE_ADJUST_ENDIANNESS_U32: &dyn Fn(u32) -> [u8; 4] = if CMS_USE_BIG_ENDIAN {&u32::to_be_bytes} else {&u32::to_le_bytes};

#[cfg(CMS_USE_BIG_ENDIAN)]
pub fn adjust_endianness_16(word: u16) -> u16 {
    word
}
#[cfg(not(CMS_USE_BIG_ENDIAN))]
pub fn adjust_endianness_16(word: u16) -> u16 {
    u16::from_be_bytes(word.to_le_bytes())
}

#[cfg(CMS_USE_BIG_ENDIAN)]
pub fn adjust_endianness_32(dword: u32) -> u32 {
    dword
}
#[cfg(not(CMS_USE_BIG_ENDIAN))]
pub fn adjust_endianness_32(dword: u32) -> u32 {
    u32::from_be_bytes(dword.to_le_bytes())
}

#[cfg(CMS_USE_BIG_ENDIAN)]
pub fn adjust_endianness_64(qword: u64) -> u64 {
    qword
}
#[cfg(not(CMS_USE_BIG_ENDIAN))]
pub fn adjust_endianness_64(qword: u64) -> u64 {
    u64::from_be_bytes(qword.to_le_bytes())
}

pub fn read_u8(reader: &mut dyn Read) -> Result<u8> {
    let mut buf = [0u8; size_of::<u8>()];
    match reader.read(&mut buf)? {
        len if len == size_of::<u8>() => Ok(buf[0]),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_u16(reader: &mut dyn Read) -> Result<u16> {
    let mut buf = [0u8; size_of::<u16>()];
    match reader.read(&mut buf)? {
        len if len == size_of::<u16>() => Ok(u16::from_be_bytes(buf)),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_u16_array(reader: &mut dyn Read, result: &mut [u16]) -> Result<()> {
    for i in 0..result.len() {
        let value = read_u16(reader)?;
        result[i] = value;
    }
    Ok(())
}

pub fn read_u32(reader: &mut dyn Read) -> Result<u32> {
    Ok(u32::from_be_bytes(read_u32_as_u8(reader)?))
}

pub fn read_u32_as_u8(reader: &mut dyn Read) -> Result<as_u8::u32> {
    let mut buf = [0u8; size_of::<u32>()];
    match reader.read(&mut buf)? {
        len if len == size_of::<u32>() => Ok(buf),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_f32(reader: &mut dyn Read) -> Result<f32> {
    let mut buf = [0u8; size_of::<f32>()];
    match reader.read(&mut buf)? {
        len if len == size_of::<f32>() => Ok(f32::from_be_bytes(buf)),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_u64(reader: &mut dyn Read) -> Result<u64> {
    let mut buf = [0u8; size_of::<u64>()];
    match reader.read(&mut buf)? {
        len if len == size_of::<u64>() => Ok(u64::from_be_bytes(buf)),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_s15f16_as_u8(reader: &mut dyn Read) -> Result<[u8; 8]> {
    let mut buf = [0u8; size_of::<S15F16>()];
    match reader.read(&mut buf)? {
        len if len == size_of::<S15F16>() => Ok(s15f16_to_f64(S15F16::from_be_bytes(buf)).to_be_bytes()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_f64(reader: &mut dyn Read) -> Result<f64> {
    let mut buf = [0u8; size_of::<f64>()];
    match reader.read(&mut buf)? {
        len if len == size_of::<f64>() => Ok(f64::from_be_bytes(buf)),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_xyz_as_u8(reader: &mut dyn Read) -> Result<as_u8::CIEXYZ> {
    let mut buf = [0u8; 24];

    buf[CIEXYZ::X].copy_from_slice(&read_s15f16_as_u8(reader)?);
    buf[CIEXYZ::Y].copy_from_slice(&read_s15f16_as_u8(reader)?);
    buf[CIEXYZ::Z].copy_from_slice(&read_s15f16_as_u8(reader)?);

    Ok(buf)
}

pub fn write_u8(writer: &mut dyn Write, value: u8) -> Result<()> {
    let buf = [value];

    match writer.write(&buf)? {
        len if len == size_of::<u8>() => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_u16(writer: &mut dyn Write, value: u16) -> Result<()> {
    let buf = u16::to_be_bytes(value);
    match writer.write(&buf)? {
        len if len == size_of::<u16>() => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_u16_array(writer: &mut dyn Write, value: &[u16]) -> Result<()> {
    for i in 0..value.len() {
        write_u16(writer, value[i])?;
    }
    Ok(())
}

pub fn write_u32(writer: &mut dyn Write, value: u32) -> Result<()> {
    write_u32_from_u8(writer, u32::to_be_bytes(value))
}

pub fn write_u32_from_u8(writer: &mut dyn Write, value: as_u8::u32) -> Result<()> {
    match writer.write(&value)? {
        len if len == size_of::<u32>() => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_f32(writer: &mut dyn Write, value: f32) -> Result<()> {
    let buf = f32::to_be_bytes(value);
    match writer.write(&buf)? {
        len if len == size_of::<f32>() => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_u64(writer: &mut dyn Write, value: u64) -> Result<()> {
    let buf = u64::to_be_bytes(value);
    match writer.write(&buf)? {
        len if len == size_of::<u64>() => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_s15f16_from_u8(writer: &mut dyn Write, value: as_u8::f64) -> Result<()> {
    let value = f64_to_s15f16(f64::from_be_bytes(value));
    let buf = S15F16::to_be_bytes(value);
    match writer.write(&buf)? {
        len if len == size_of::<S15F16>() => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_xyz_from_u8(writer: &mut dyn Write, value: as_u8::CIEXYZ) -> Result<()> {
    write_s15f16_from_u8(writer, value[CIEXYZ::X].try_into().unwrap())?;
    write_s15f16_from_u8(writer, value[CIEXYZ::Y].try_into().unwrap())?;
    write_s15f16_from_u8(writer, value[CIEXYZ::Z].try_into().unwrap())?;

    Ok(())
}

pub fn write_f64(writer: &mut dyn Write, value: f64) -> Result<()> {
    let buf = f64::to_be_bytes(value);
    match writer.write(&buf)? {
        len if len == size_of::<f64>() => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn u8f8_to_f64(fixed8: U8F8) -> f64 {
    let (msb, lsb) = (((fixed8 >> 8) & 0xFF) as u8, (fixed8 & 0xFF) as u8);

    (msb as f64) + ((lsb as f64) / 256.0)
}
pub fn f64_to_u8f8(val: f64) -> U8F8 {
    let fixed32 = f64_to_s15f16(val);

    ((fixed32 >> 8) & 0xFFFF) as U8F8
}
pub fn s15f16_to_f64(fixed32: S15F16) -> f64 {
    let sign = if fixed32 < 0 { -1.0 } else { 1.0 };
    let fixed32 = S15F16::abs(fixed32);

    let whole = ((fixed32 >> 16) & 0xFFFF) as u16;
    let frac_part = (fixed32 & 0xFFFF) as u16;

    let mid = (frac_part as f64 / 65536.0) as f64;
    let floater = whole as f64 + mid as f64;

    sign * floater
}
pub fn f64_to_s15f16(v: f64) -> S15F16 {
    f64::floor(v * 65536.0 + 0.5) as S15F16
}

pub struct PluginBase<'a> {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
    pub next: &'a PluginBase<'a>,
}

pub const MAX_TYPES_IN_LCMS_PLUGIN: u8 = 20;

/* ---------------------------------------------------- Tag type ---------------------------------------------------- */

pub type TagTypeRead = fn(&mut dyn Read, items: &mut [u8], tag_size: usize) -> Result<usize>;
pub type TagTypeWrite = fn(writer: &mut dyn Write, items: &[u8], count: usize) -> Result<()>;

pub struct TagTypeHandler {
    pub signature: Signature,
    pub version: u32,

    pub read: TagTypeRead,
    pub write: TagTypeWrite,
}

/* ------------------------------------------------------ Tags ------------------------------------------------------ */

pub type DecideType = fn(f64) -> Signature;

pub struct TagDescriptor {
    /// If this tag needs to be an array, how many elements should keep
    pub element_count: u32,

    pub supported_types: &'static [Signature],

    pub decide_type: Option<DecideType>,
}

pub struct PluginTag<'a> {
    pub base: PluginBase<'a>,
    pub signature: Signature,
    pub descriptor: TagDescriptor,
}

/* ------------------------------------------------- Full Transform ------------------------------------------------- */

pub struct Stride {
    pub bytes_per_line_in: u32,
    pub bytes_per_line_out: u32,
    pub bytes_per_plane_in: u32,
    pub bytes_per_plane_out: u32,
}
