#![allow(dead_code)]

pub const LCMS_VERSION: i32 = 2120;

// Types

pub type U8F8 = u16;
pub type S15F16 = i32;
pub type U16F16 = u32;

mod signature;
pub use signature::Signature;

pub const USE_BIG_ENDIAN: bool = if cfg!(BIG_ENDIAN = "true") {
    true
} else {
    false
};

/// D50 XYZ normalized to Y=1.0
pub mod d50 {
    pub const X: f64 = 0.9642;
    pub const Y: f64 = 1.0;
    pub const Z: f64 = 0.8249;
}

/// V4 perceptual black
pub mod perceptual_black {
    pub const X: f64 = 0.00336;
    pub const Y: f64 = 0.0034731;
    pub const Z: f64 = 0.00287;
}

pub mod signatures;

// Device attributes, currently defined values correspond to the low 4 bytes of the 8 byte attribute quantity
pub mod device_attributes {
    pub const REFLECTIVE: u32 = 0;
    pub const TRANSPARANCY: u32 = 1;
    pub const GLOSSY: u32 = 0;
    pub const MATTE: u32 = 2;
}

/// Common structures in ICC tags
pub struct ICCData {
    pub length: u32,
    pub flag: u32,
    pub data: [u8],
}

/// ICC date time
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DateTimeNumber {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
}

/// ICC XYZ
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EncodedXYZNumber {
    pub x: S15F16,
    pub y: S15F16,
    pub z: S15F16,
}

/// Profile ID as computed by MD5 algorithm
#[derive(Copy, Clone, Eq)]
pub union ProfileID {
    pub id8: [u8; 16],
    pub id16: [u16; 8],
    pub id32: [u32; 4],
}

impl std::fmt::Debug for ProfileID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut value = String::with_capacity(64);
        for i in 0..16 {
            unsafe {
                value.push_str(format!(" {:2x} ", self.id8[i]).as_str());
            }
        }

        f.debug_struct("ProfileID").field("value", &value).finish()
    }
}
impl PartialEq for ProfileID {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            self.id8 == other.id8
        }
    }
}

/// Profile Header -- 32-bit aligned
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ICCHeader {
    /// Profile size in bytes
    pub size: u32,
    /// CMM for this profile
    pub cmm_id: Signature,
    /// Format version number
    pub version: u32,
    /// Type of profile
    pub device_class: Signature,
    /// Color space of data
    pub color_space: Signature,
    /// PCS, XYZ or LAB only
    pub pcs: Signature,
    /// Date profile was created
    pub date: DateTimeNumber,
    /// Magic Number to identity an ICC profile
    pub magic: Signature,
    /// Primary platform
    pub platform: Signature,
    /// Various bit settings
    pub flags: u32,
    /// Device manufacturer
    pub manufacturer: Signature,
    /// Device model Number
    pub model: u32,
    /// Device attributes
    pub attributes: u64,
    /// Rendering intent
    pub rendering_intent: u32,
    /// Profile illuminant
    pub illuminant: EncodedXYZNumber,
    /// Profile creator
    pub creator: Signature,
    /// Profile ID using MD5
    pub profile_id: ProfileID,
    /// Reserved for future use
    pub reserved: [u8; 28],
}

/// A tag entry in directory
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TagEntry {
    /// The tag signature
    pub signature: Signature,
    /// Start of tag
    pub offset: u32,
    /// Size in bytes
    pub size: u32,
}

pub const MAX_CHANNELS: u32 = 16;

mod pixel_type;
pub use pixel_type::PixelType;

pub enum ColorSpace {
    Any = 0,
    // 1 & 2 are reserved
    Gray = 3,
    Rgb = 4,
    Cmy = 5,
    Cmyk = 6,
    YCbCr = 7,
    Yuv = 8,
    Xyz = 9,
    Lab = 10,
    Yuvk = 11,
    Hsv = 12,
    Hls = 13,
    Yxy = 14,

    Mch1 = 15,
    Mch2 = 16,
    Mch3 = 17,
    Mch4 = 18,
    Mch5 = 19,
    Mch6 = 20,
    Mch7 = 21,
    Mch8 = 22,
    Mch9 = 23,
    Mch10 = 24,
    Mch11 = 25,
    Mch12 = 26,
    Mch13 = 27,
    Mch14 = 28,
    Mch15 = 29,

    LabV2 = 30,
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CIEXYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CIExyY {
    pub x: f64,
    pub y: f64,
    pub Y: f64,
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CIELab {
    pub L: f64,
    pub a: f64,
    pub b: f64,
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CIELCh {
    pub L: f64,
    pub C: f64,
    pub h: f64,
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CIEJCh {
    pub J: f64,
    pub C: f64,
    pub h: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CIEXYZTriple {
    pub red: CIEXYZ,
    pub green: CIEXYZ,
    pub blue: CIEXYZ,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CIExyYTripple {
    pub red: CIExyY,
    pub green: CIExyY,
    pub blue: CIExyY,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ICCMeasurementConditions {
    pub observer: u32,
    pub backing: CIEXYZ,
    pub geometry: u32,
    pub flare: f64,
    pub illuminant_type: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ICCViewingConditions {
    pub illuminant_xyz: CIEXYZ,
    pub surround_xyz: CIEXYZ,
    pub illuminant_type: u32,
}

pub mod illuminant_type {
    pub const UNKNOWN: u32 = 0;
    pub const D50: u32 = 1;
    pub const D65: u32 = 2;
    pub const D93: u32 = 3;
    pub const F2: u32 = 4;
    pub const D55: u32 = 5;
    pub const A: u32 = 6;
    pub const E: u32 = 7;
    pub const F8: u32 = 8;
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ViewingConditions {
    pub white_point: CIEXYZ,
    pub Yb: f64,
    pub La: f64,
    pub surround: u32,
    pub D_value: f64,
}

/// Tone curves
///
/// This describes a curve segment. Users can increase the nuber of available types by using a proper plug-in.
/// Parametric segments allow 10 parameters at most
pub struct CurveSegment {
    pub x0: f32,
    pub x1: f32,
    pub r#type: i32,
    pub params: [f64; 10],
    pub n_grid_points: u32,
    pub sampled_points: [f32],
}

mod internal;
pub mod plugin;
