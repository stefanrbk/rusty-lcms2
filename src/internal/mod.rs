use crate::Signature;

pub mod types;
pub mod tags;
pub mod mlu;

pub const MAX_ENCODABLE_XYZ: f64 = 1.0 + 32767.0 / 32768.0;
pub const MIN_ENCODABLE_AB2: f64 = -128.0;
pub const MAX_ENCODABLE_AB2: f64 = (65535.0 / 256.0) - 128.0;
pub const MIN_ENCODABLE_AB4: f64 = -128.0;
pub const MAX_ENCODABLE_AB4: f64 = 127.0;

pub const MAX_STAGE_CHANNELS: usize = 128;

// Fast bit conversion here

pub const MATRIX_DET_TOLERANCE: f64 = 0.0001;

// Fixed point functions here

pub mod fast_floor;

// Some broken types
const CORBIS_BROKEN_XYZ_TYPE: Signature = Signature::new(&[0x17, 0xA5, 0x05, 0xB8]);
const MONACO_BROKEN_CURVE_TYPE: Signature = Signature::new(&[0x94, 0x78, 0xEE, 0x00]);
