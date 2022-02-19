use bitfield::bitfield;

pub const LCMS_VERSION: i32 = 2120;

// Types

pub type U8F8 = u16;
pub type S15F16 = i32;
pub type U16F16 = u32;

pub struct CmsSignature(u32);
mod signature;

pub const CMS_USE_BIG_ENDIAN: bool = if cfg!(BIG_ENDIAN = "true") {
    true
} else {
    false
};

// D50 XYZ normalized to Y=1.0
pub const CMS_D50_X: f64 = 0.9642;
pub const CMS_D50_Y: f64 = 1.0;
pub const CMS_D50_Z: f64 = 0.8249;

// V4 perceptual black
pub const CMS_PERCEPTUAL_BLACK_X: f64 = 0.00336;
pub const CMS_PERCEPTUAL_BLACK_Y: f64 = 0.0034731;
pub const CMS_PERCEPTUAL_BLACK_Z: f64 = 0.00287;

pub mod signatures;

// Device attributes, currently defined values correspond to the low 4 bytes of the 8 byte attribute quantity
pub const CMS_REFLECTIVE: u32 = 0;
pub const CMS_TRANSPARANCY: u32 = 1;
pub const CMS_GLOSSY: u32 = 0;
pub const CMS_MATTE: u32 = 2;

/// Common structures in ICC tags
pub struct CmsICCData {
    pub length: u32,
    pub flag: u32,
    pub data: [u8],
}

/// ICC date time
pub struct CmsDateTimeNumber {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
}

/// ICC XYZ
pub struct CmsEncodedXYZNumber {
    pub x: S15F16,
    pub y: S15F16,
    pub z: S15F16,
}

/// Profile ID as computed by MD5 algorithm
pub union CmsProfileID {
    pub id8: [u8; 16],
    pub id16: [u16; 8],
    pub id32: [u32; 4],
}

/// Profile Header -- 32-bit aligned
pub struct CmsICCHeader {
    /// Profile size in bytes
    pub size: u32,
    /// CMM for this profile
    pub cmm_id: CmsSignature,
    /// Format version number
    pub version: u32,
    /// Type of profile
    pub device_class: CmsSignature,
    /// Color space of data
    pub color_space: CmsSignature,
    /// PCS, XYZ or LAB only
    pub pcs: CmsSignature,
    /// Date profile was created
    pub date: CmsDateTimeNumber,
    /// Magic Number to identity an ICC profile
    pub magic: CmsSignature,
    /// Primary platform
    pub platform: CmsSignature,
    /// Various bit settings
    pub flags: u32,
    /// Device manufacturer
    pub manufacturer: CmsSignature,
    /// Device model Number
    pub model: u32,
    /// Device attributes
    pub attributes: u64,
    /// Rendering intent
    pub rendering_intent: u32,
    /// Profile illuminant
    pub illuminant: CmsEncodedXYZNumber,
    /// Profile creator
    pub creator: CmsSignature,
    /// Profile ID using MD5
    pub profile_id: CmsProfileID,
    /// Reserved for future use
    pub reserved: [u8; 28],
}

/// A tag entry in directory
pub struct CmsTagEntry {
    /// The tag signature
    pub signature: CmsSignature,
    /// Start of tag
    pub offset: u32,
    /// Size in bytes
    pub size: u32,
}

bitfield! {
    /// Format of pixel is defined by one cmsUInt32Number, using bit fields as follows
    ///
    ///                                   2                1          0
    ///                            4 3 2 10987 6 5 4 3 2 1 098 7654 321
    ///                            M A O TTTTT U Y F P X S EEE CCCC BBB
    ///
    ///                M: Premultiplied alpha (only works when extra samples is 1)
    ///                A: Floating point -- With this flag we can differentiate 16 bits as float and as int
    ///                O: Optimized -- previous optimization already returns the final 8-bit value
    ///                T: Pixeltype
    ///                F: Flavor  0=MinIsBlack(Chocolate) 1=MinIsWhite(Vanilla)
    ///                P: Planar? 0=Chunky, 1=Planar
    ///                X: swap 16 bps endianness?
    ///                S: Do swap? ie, BGR, KYMC
    ///                E: Extra samples
    ///                C: Channels (Samples per pixel)
    ///                B: bytes per sample
    ///                Y: Swap first - changes ABGR to BGRA and KCMY to CMYK
    pub struct CmsPixelType(u32);
    pub u8, bps, set_bps: 2, 0;
    pub u8, channels, set_channels: 6, 3;
    pub u8, extra, set_extra: 9, 7;
    pub do_swap, set_do_swap: 10;
    pub endian16, set_endian16: 11;
    pub planar, set_planar: 12;
    pub flavor, set_flavor: 13;
    pub swap_first, set_swap_first: 14;
    u8, _color_space, _set_color_space: 20, 16;
    pub optimized, set_optimized: 21;
    pub float, set_float: 22;
    pub premul, set_premul: 23;
}
mod pixel_type;

pub enum CmsColorSpace {
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
pub struct CmsCIEXYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

#[allow(non_snake_case)]
pub struct CmsCIExyY {
    pub x: f64,
    pub y: f64,
    pub Y: f64,
}

#[allow(non_snake_case)]
pub struct CmsCIELab {
    pub L: f64,
    pub a: f64,
    pub b: f64,
}

#[allow(non_snake_case)]
pub struct CmsCIELCh {
    pub L: f64,
    pub C: f64,
    pub h: f64,
}

#[allow(non_snake_case)]
pub struct CmsCIEJCh {
    pub J: f64,
    pub C: f64,
    pub h: f64,
}

pub struct CmsCIEXYZTriple {
    pub red: CmsCIEXYZ,
    pub green: CmsCIEXYZ,
    pub blue: CmsCIEXYZ,
}

pub struct CmsCIExyYTripple {
    pub red: CmsCIExyY,
    pub green: CmsCIExyY,
    pub blue: CmsCIExyY,
}

pub struct CmsICCMeasurementConditions {
    pub observer: u32,
    pub backing: CmsCIEXYZ,
    pub geometry: u32,
    pub flare: f64,
    pub illuminant_type: u32,
}

pub struct CmsICCViewingConditions {
    pub illuminant_xyz: CmsCIEXYZ,
    pub surround_xyz: CmsCIEXYZ,
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
pub struct CmsViewingConditions {
    pub white_point: CmsCIEXYZ,
    pub Yb: f64,
    pub La: f64,
    pub surround: u32,
    pub D_value: f64,
}

/// Tone curves
/// 
/// This describes a curve segment. Users can increase the nuber of available types by using a proper plug-in.
/// Parametric segments allow 10 parameters at most
pub struct CmsCurveSegment {
    pub x0: f32,
    pub x1: f32,
    pub r#type: i32,
    pub params: [f64; 10],
    pub n_grid_points: u32,
    pub sampled_points: [f32]
}

pub mod plugin;
mod internal;
