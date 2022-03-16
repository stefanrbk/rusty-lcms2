#![macro_use]

use std::convert::TryInto;
use std::io::*;
use std::mem::size_of;
use std::sync::Arc;

use crate::*;

mod big_endian;
mod chunks;
mod context;
mod error;
mod little_endian;
mod mat3;
mod tag_base;
mod tags;
mod vec3;

#[cfg(test)]
mod tests;

// Public exports
pub use context::Context;
pub use mat3::Mat3;
pub use tag_base::TagBase;
pub use vec3::Vec3;

#[cfg(feature = "use_little_endian")]
pub use little_endian::adjust_endianness_16;
#[cfg(feature = "use_little_endian")]
pub use little_endian::adjust_endianness_32;
#[cfg(feature = "use_little_endian")]
pub use little_endian::adjust_endianness_64;

#[cfg(not(feature = "use_little_endian"))]
pub use big_endian::adjust_endianness_16;
#[cfg(not(feature = "use_little_endian"))]
pub use big_endian::adjust_endianness_32;
#[cfg(not(feature = "use_little_endian"))]
pub use big_endian::adjust_endianness_64;

// const READ_ADJUST_ENDIANNESS_U32: &dyn Fn([u8; 4]) -> u32 = if CMS_USE_BIG_ENDIAN {&u32::from_be_bytes} else {&u32::from_le_bytes};
// const WRITE_ADJUST_ENDIANNESS_U32: &dyn Fn(u32) -> [u8; 4] = if CMS_USE_BIG_ENDIAN {&u32::to_be_bytes} else {&u32::to_le_bytes};

pub fn read_u8(reader: &mut dyn Read) -> Result<u8> {
    const SIZE: usize = size_of::<u8>();

    let mut buf = [0u8; SIZE];
    match reader.read(&mut buf)? {
        len if len == SIZE => Ok(buf[0]),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_u16(reader: &mut dyn Read) -> Result<u16> {
    const SIZE: usize = size_of::<u16>();

    let mut buf = [0u8; SIZE];
    match reader.read(&mut buf)? {
        len if len == SIZE => Ok(if use_big_endian!() {
            u16::from_be_bytes(buf)
        } else {
            u16::from_le_bytes(buf)
        }),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_u16_array(reader: &mut dyn Read, count: usize) -> Result<Box<[u16]>> {
    let mut result = vec![0u16; count];
    for i in 0..count {
        let value = read_u16(reader)?;
        result[i] = value;
    }
    Ok(result.into_boxed_slice())
}

pub fn read_u32(reader: &mut dyn Read) -> Result<u32> {
    let mut buf = [0u8; size_of::<u32>()];
    match reader.read(&mut buf)? {
        len if len == size_of::<u32>() => Ok(if use_big_endian!() {
            u32::from_be_bytes(buf)
        } else {
            u32::from_le_bytes(buf)
        }),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_f32(reader: &mut dyn Read) -> Result<f32> {
    const SIZE: usize = size_of::<f32>();

    let mut buf = [0u8; SIZE];
    match reader.read(&mut buf)? {
        len if len == SIZE => Ok(if use_big_endian!() {
            f32::from_be_bytes(buf)
        } else {
            f32::from_le_bytes(buf)
        }),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_u64(reader: &mut dyn Read) -> Result<u64> {
    const SIZE: usize = size_of::<u64>();

    let mut buf = [0u8; SIZE];
    match reader.read(&mut buf)? {
        len if len == SIZE => Ok(if use_big_endian!() {
            u64::from_be_bytes(buf)
        } else {
            u64::from_le_bytes(buf)
        }),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

/// Reads a [`S15F16`], converts it to [`f64`], and returns the corresponding value.
///
/// ```
/// # use lcms2::plugin::read_s15f16;
/// # use lcms2::plugin::f64_to_s15f16;
/// # let mut value: [u8; 4] = if cfg!(feature = "use_little_endian") {
/// #     f64_to_s15f16(2.5f64).to_le_bytes() } else {
/// #     f64_to_s15f16(2.5f64).to_be_bytes() };
/// // value = 2.5 in fixed point S15F16 as a [u8; 4]
/// assert_eq!(
///     read_s15f16(&mut value.as_slice()).unwrap(),
///     2.5f64
/// )
/// ```
pub fn read_s15f16(reader: &mut dyn Read) -> Result<f64> {
    const SIZE: usize = size_of::<S15F16>();

    let mut buf = [0u8; SIZE];
    match reader.read(&mut buf)? {
        len if len == SIZE => {
            let fixed_value = if use_big_endian!() {
                S15F16::from_be_bytes(buf)
            } else {
                S15F16::from_le_bytes(buf)
            };
            let as_f64 = s15f16_to_f64(fixed_value);
            Ok(as_f64)
        }
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn read_f64(reader: &mut dyn Read) -> Result<f64> {
    const SIZE: usize = size_of::<f64>();

    let mut buf = [0u8; SIZE];
    match reader.read(&mut buf)? {
        len if len == SIZE => Ok(if use_big_endian!() {
            f64::from_be_bytes(buf)
        } else {
            f64::from_le_bytes(buf)
        }),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

/// Reads 3 [`S15F16`] values, converts them to [`f64`] values, and returns them in a [`CIEXYZ`].
///
/// ```
/// # use lcms2::CIEXYZ;
/// # use lcms2::plugin::read_xyz;
/// # use lcms2::plugin::f64_to_s15f16;
/// # let x = if cfg!(feature = "use_little_endian") {
/// #     f64_to_s15f16(2.5f64).to_le_bytes() } else {
/// #     f64_to_s15f16(2.5f64).to_be_bytes() };
/// # let y = if cfg!(feature = "use_little_endian") {
/// #     f64_to_s15f16(42.0f64).to_le_bytes() } else {
/// #     f64_to_s15f16(42.0f64).to_be_bytes() };
/// # let z = if cfg!(feature = "use_little_endian") {
/// #     f64_to_s15f16(-1.0f64).to_le_bytes() } else {
/// #     f64_to_s15f16(-1.0f64).to_be_bytes() };
/// # let mut value = [0u8; 12];
/// # value[0..4].copy_from_slice(&x);
/// # value[4..8].copy_from_slice(&y);
/// # value[8..12].copy_from_slice(&z);
/// // value = [2.5, 42.0, -1.0] in fixed point S15F16 values as a [u8; 12]
///
/// assert_eq!(
///     read_xyz(&mut value.as_slice()).unwrap(),
///     CIEXYZ { X: 2.5f64, Y: 42.0f64, Z: -1.0f64 }
/// )
/// ```
pub fn read_xyz(reader: &mut dyn Read) -> Result<CIEXYZ> {
    Ok(CIEXYZ {
        X: read_s15f16(reader)?,
        Y: read_s15f16(reader)?,
        Z: read_s15f16(reader)?,
    })
}

pub fn write_u8(writer: &mut dyn Write, value: u8) -> Result<()> {
    const SIZE: usize = size_of::<u8>();

    let buf = [value];

    match writer.write(&buf)? {
        len if len == SIZE => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_u16(writer: &mut dyn Write, value: u16) -> Result<()> {
    const SIZE: usize = size_of::<u16>();

    match if use_big_endian!() {
        writer.write(&value.to_be_bytes())
    } else {
        writer.write(&value.to_le_bytes())
    }? {
        len if len == SIZE => Ok(()),
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
    const SIZE: usize = size_of::<u32>();

    match if use_big_endian!() {
        writer.write(&value.to_be_bytes())
    } else {
        writer.write(&value.to_le_bytes())
    }? {
        len if len == SIZE => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_f32(writer: &mut dyn Write, value: f32) -> Result<()> {
    const SIZE: usize = size_of::<f32>();

    match if use_big_endian!() {
        writer.write(&value.to_be_bytes())
    } else {
        writer.write(&value.to_le_bytes())
    }? {
        len if len == SIZE => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_u64(writer: &mut dyn Write, value: u64) -> Result<()> {
    const SIZE: usize = size_of::<u64>();

    match if use_big_endian!() {
        writer.write(&value.to_be_bytes())
    } else {
        writer.write(&value.to_le_bytes())
    }? {
        len if len == SIZE => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

/// Converts a [`f64`] to a [`S15F16`] and writes out the result.
///
/// ```
/// # use lcms2::plugin::write_s15f16;
/// # use lcms2::plugin::f64_to_s15f16;
/// # let mut buf = [0u8; 4];
/// # let result = if cfg!(feature = "use_little_endian") {
/// #     f64_to_s15f16(2.5f64).to_le_bytes() } else {
/// #     f64_to_s15f16(2.5f64).to_be_bytes() };
/// write_s15f16(&mut buf.as_mut_slice(), 2.5f64);
/// assert_eq!(buf, result);
/// ```
pub fn write_s15f16(writer: &mut dyn Write, value: f64) -> Result<()> {
    const SIZE: usize = size_of::<S15F16>();

    match if use_big_endian!() {
        writer.write(&f64_to_s15f16(value).to_be_bytes())
    } else {
        writer.write(&f64_to_s15f16(value).to_le_bytes())
    }? {
        len if len == SIZE => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

pub fn write_xyz(writer: &mut dyn Write, value: CIEXYZ) -> Result<()> {
    write_s15f16(writer, value.X)?;
    write_s15f16(writer, value.Y)?;
    write_s15f16(writer, value.Z)?;

    Ok(())
}

pub fn write_f64(writer: &mut dyn Write, value: f64) -> Result<()> {
    const SIZE: usize = size_of::<f64>();

    match if use_big_endian!() {
        writer.write(&f64_to_s15f16(value).to_be_bytes())
    } else {
        writer.write(&f64_to_s15f16(value).to_le_bytes())
    }? {
        len if len == SIZE => Ok(()),
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

/// Converts a [`S15F16`] to a [`f64`]
///
/// ```
/// # use lcms2::plugin::s15f16_to_f64;
/// let value = 0x0002_8000i32; // 2.5 in fixed point
/// assert_eq!(s15f16_to_f64(value), 2.5f64);
/// ```
///
/// ```
/// # use lcms2::plugin::s15f16_to_f64;
/// let value = 0x002A_0000i32; // 42.0 in fixed point
/// assert_eq!(s15f16_to_f64(value), 42.0f64);
/// ```
///
/// ```
/// # use lcms2::plugin::s15f16_to_f64;
/// let value = -0x0001_0000i32; // -1.0 in fixed point
/// assert_eq!(s15f16_to_f64(value), -1.0f64);
/// ```
pub fn s15f16_to_f64(fixed32: S15F16) -> f64 {
    let sign = if fixed32 < 0 { -1.0 } else { 1.0 };
    let fixed32 = S15F16::abs(fixed32);

    let whole = ((fixed32 >> 16) & 0xFFFF) as u16;
    let frac_part = (fixed32 & 0xFFFF) as u16;

    let mid = (frac_part as f64 / 65536.0) as f64;
    let floater = whole as f64 + mid as f64;

    sign * floater
}

/// Converts a [`f64`] to a [`S15F16`]
///
/// ```
/// # use lcms2::plugin::f64_to_s15f16;
/// let value = 0x0002_8000i32; // 2.5 in fixed point
/// assert_eq!(f64_to_s15f16(2.5f64), value);
/// ```
///
/// ```
/// # use lcms2::plugin::f64_to_s15f16;
/// let value = 0x002A_0000i32; // 42.0 in fixed point
/// assert_eq!(f64_to_s15f16(42.0f64), value);
/// ```
///
/// ```
/// # use lcms2::plugin::f64_to_s15f16;
/// let value = -0x0001_0000i32; // -1.0 in fixed point
/// assert_eq!(f64_to_s15f16(-1.0f64), value);
/// ```
pub fn f64_to_s15f16(v: f64) -> S15F16 {
    f64::floor(v * 65536.0 + 0.5) as S15F16
}

/* ---------------------------------------------------- Context ----------------------------------------------------- */

pub struct Plugin {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
    pub next: Option<Arc<Plugin>>,
    pub data: PluginType,
}

pub const MAX_TYPES_IN_LCMS_PLUGIN: u8 = 20;

pub enum PluginType {
    Tag { sig: Signature, desc: TagDescriptor },
}

/* ---------------------------------------------------- Tag Type ---------------------------------------------------- */

pub type TagTypeRead = fn(&mut dyn Read, tag_size: usize) -> Result<(usize, Box<[u8]>)>;
pub type TagTypeWrite = fn(writer: &mut dyn Write, items: &[u8], count: usize) -> Result<()>;

pub struct TagTypeHandler {
    pub signature: Signature,
    pub version: u32,

    pub read: TagTypeRead,
    pub write: TagTypeWrite,
}

/* ------------------------------------------------------ Tags ------------------------------------------------------ */

pub use tags::TagDescriptor;
pub use tags::TagListItem;

#[macro_export]
macro_rules! TagListItem {
    ($signature: expr, $element_count:expr, $supported_types:expr) => {
        TagListItem {
            sig: $signature,
            desc: TagDescriptor {
                element_count: $element_count,
                supported_types: &$supported_types,
                decide_type: None,
            },
        }
    };
    ($signature: expr, $element_count:expr, $supported_types:expr, $decide_type:expr) => {
        TagListItem {
            sig: $signature,
            desc: TagDescriptor {
                element_count: $element_count,
                supported_types: &$supported_types,
                decide_type: Some($decide_type),
            },
        }
    };
}

pub type DecideType = fn(f64, &[i8]) -> Signature;

/* ------------------------------------------------- Full Transform ------------------------------------------------- */

pub struct Stride {
    pub bytes_per_line_in: u32,
    pub bytes_per_line_out: u32,
    pub bytes_per_plane_in: u32,
    pub bytes_per_plane_out: u32,
}
