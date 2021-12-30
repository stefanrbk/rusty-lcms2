use bitfield::bitfield;

pub const LCMS_VERSION: i32 = 2120;

// Types

pub type U8F8 = u16;
pub type S15F16 = i32;
pub type U16F16 = u32;

pub struct CmsSignature(u32);
impl CmsSignature {
    pub const fn new(value: &[u8; 4]) -> CmsSignature {
        CmsSignature(u32::from_be_bytes(*value))
    }
}
impl From<u32> for CmsSignature {
    fn from(item: u32) -> Self {
        CmsSignature(item)
    }
}
impl From<&[u8; 4]> for CmsSignature {
    fn from(value: &[u8; 4]) -> Self {
        CmsSignature(u32::from_be_bytes(*value))
    }
}
impl From<&[u8; 3]> for CmsSignature {
    fn from(value: &[u8; 3]) -> Self {
        let mut result: [u8; 4] = [0x20; 4];
        result[..3].copy_from_slice(&*value);
        CmsSignature(u32::from_be_bytes(result))
    }
}
impl From<&[u8; 2]> for CmsSignature {
    fn from(value: &[u8; 2]) -> Self {
        let mut result: [u8; 4] = [0x20; 4];
        result[..2].copy_from_slice(&*value);
        CmsSignature(u32::from_be_bytes(result))
    }
}
impl From<&[u8; 1]> for CmsSignature {
    fn from(value: &[u8; 1]) -> Self {
        let mut result: [u8; 4] = [0x20; 4];
        result[..1].copy_from_slice(&*value);
        CmsSignature(u32::from_be_bytes(result))
    }
}
impl From<&[u8]> for CmsSignature {
    fn from(value: &[u8]) -> Self {
        let len = value.len();
        let mut result: [u8; 4] = [0x20; 4];
        if len >= 4 {
            result.copy_from_slice(&value[..4]);
            CmsSignature::from(&result)
        } else if len == 3 {
            result[..3].copy_from_slice(&value[..3]);
            CmsSignature::from(&result)
        } else if len == 2 {
            result[..2].copy_from_slice(&value[..2]);
            CmsSignature::from(&result)
        } else if len == 1 {
            result[..1].copy_from_slice(&value[..1]);
            CmsSignature::from(&result)
        } else {
            panic!()
        }
    }
}
impl From<&str> for CmsSignature {
    fn from(s: &str) -> Self {
        CmsSignature::from(s.as_bytes())
    }
}

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

// Common structures in ICC tags
pub struct CmsICCData {
    pub length: u32,
    pub flag: u32,
    pub data: [u8],
}

// ICC date time
pub struct CmsDateTimeNumber {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
}

// ICC XYZ
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

/// ICC base tag
pub struct CmsTagBase {
    pub signature: CmsSignature,
    pub reserved: [u8; 4],
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

macro_rules! PixelTypeDef {
    ($($name:ident)*; $($rest:tt)*) => {
        $($name)*: CmsPixelType = PixelType!($($rest)*);
    };
}

macro_rules! PixelType {
    (set) => (0);
    (set float) => (
        (1 << 22)
    );
    (set optimized) => (
        (1 << 21)
    );
    (set color_space $value:expr) => (
        ((($value as u32) & 31) << 16)
    );
    (set swap_first) => (
        (1 << 14)
    );
    (set min_is_white) => (
        (1 << 13)
    );
    (set planar) => (
        (1 << 12)
    );
    (set endian16) => (
        (1 << 11)
    );
    (set do_swap) => (
        (1 << 10)
    );
    (set extra $value:expr) => (
        ((($value as u32) & 7) << 7)
    );
    (set channels $value:expr) => (
        ((($value as u32) & 15) << 3)
    );
    (set bps $value:expr) => (
        (($value as u32) & 7)
    );


    (set $head:ident $($val:expr)?, $($tail1:ident $($tail2:expr)?),*) => (
        (PixelType!(set $head $($val)?) | PixelType!(set $($tail1 $($tail2)?),*))
    );
    ($head:ident $($val:expr)?) => (
        CmsPixelType { 0: PixelType!(set $head $($val)?)}
    );
    ($head:ident $($val:expr)?, $($tail1:ident $($tail2:expr)?),*) => (
        CmsPixelType { 0: PixelType!(set $head $($val)?, $($tail1 $($tail2)?),*)}
    );
}

bitfield! {
    /// Format of pixel is defined by one cmsUInt32Number, using bit fields as follows
    ///
    ///                                   2                1          0
    ///                              3 2 10987 6 5 4 3 2 1 098 7654 321
    ///                              A O TTTTT U Y F P X S EEE CCCC BBB
    ///
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
}

impl CmsPixelType {
    PixelTypeDef!(pub const GRAY_8; color_space CmsColorSpace::Gray, channels 1, bps 1);
    PixelTypeDef!(pub const GRAY_8_REV; color_space CmsColorSpace::Gray, channels 1, bps 1, min_is_white);
    PixelTypeDef!(pub const GRAY_16; color_space CmsColorSpace::Gray,channels 1, bps 2);
    PixelTypeDef!(pub const GRAY_16_REV; color_space CmsColorSpace::Gray,channels 1, bps 2, min_is_white);
    PixelTypeDef!(pub const GRAY_16_SE; color_space CmsColorSpace::Gray,channels 1, bps 2, endian16);
    PixelTypeDef!(pub const GRAYA_8; color_space CmsColorSpace::Gray, extra 1, channels 1, bps 1);
    PixelTypeDef!(pub const GRAYA_16; color_space CmsColorSpace::Gray, extra 1, channels 1, bps 2);
    PixelTypeDef!(pub const GRAYA_16_SE; color_space CmsColorSpace::Gray, extra 1, channels 1, bps 2, endian16);
    PixelTypeDef!(pub const GRAYA_8_PLANAR; color_space CmsColorSpace::Gray, extra 1, channels 1, bps 1, planar);
    PixelTypeDef!(pub const GRAYA_16_PLANAR; color_space CmsColorSpace::Gray, extra 1, channels 1, bps 2, planar);

    PixelTypeDef!(pub const RGB_8; color_space CmsColorSpace::Rgb, channels 3, bps 1);
    PixelTypeDef!(pub const RGB_8_PLANAR; color_space CmsColorSpace::Rgb, channels 3, bps 1, planar);
    PixelTypeDef!(pub const BGR_8; color_space CmsColorSpace::Rgb, channels 3, bps 1, do_swap);
    PixelTypeDef!(pub const BGR_8_PLANAR; color_space CmsColorSpace::Rgb, channels 3, bps 1, do_swap, planar);
    PixelTypeDef!(pub const RGB_16; color_space CmsColorSpace::Rgb, channels 3, bps 2);
    PixelTypeDef!(pub const RGB_16_PLANAR; color_space CmsColorSpace::Rgb, channels 3, bps 2, planar);
    PixelTypeDef!(pub const RGB_16_SE; color_space CmsColorSpace::Rgb, channels 3, bps 2, endian16);
    PixelTypeDef!(pub const BGR_16; color_space CmsColorSpace::Rgb, channels 3, bps 2, do_swap);
    PixelTypeDef!(pub const BGR_16_PLANAR; color_space CmsColorSpace::Rgb, channels 3, bps 2, do_swap, planar);
    PixelTypeDef!(pub const BGR_16_SE; color_space CmsColorSpace::Rgb, channels 3, bps 2, do_swap, endian16);
    PixelTypeDef!(pub const RGBA_8; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1);
    PixelTypeDef!(pub const RGBA_8_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, planar);
    PixelTypeDef!(pub const RGBA_16; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2);
    PixelTypeDef!(pub const RGBA_16_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, planar);
    PixelTypeDef!(pub const RGBA_16_SE; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, endian16);

    PixelTypeDef!(pub const ARGB_8; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, swap_first);
    PixelTypeDef!(pub const ARGB_8_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, swap_first, planar);
    PixelTypeDef!(pub const ARGB_16; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, swap_first);
    PixelTypeDef!(pub const ABGR_8; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap);
    PixelTypeDef!(pub const ABGR_8_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, planar);
    PixelTypeDef!(pub const ABGR_16; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap);
    PixelTypeDef!(pub const ABGR_16_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, planar);
    PixelTypeDef!(pub const ABGR_16_SE; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, endian16);
    PixelTypeDef!(pub const BGRA_8; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, swap_first);
    PixelTypeDef!(pub const BGRA_8_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, swap_first, planar);
    PixelTypeDef!(pub const BGRA_16; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, swap_first);
    PixelTypeDef!(pub const BGRA_16_SE; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, endian16, do_swap, swap_first);
    PixelTypeDef!(pub const CMY_8; color_space CmsColorSpace::Cmy, channels 3, bps 1);
    PixelTypeDef!(pub const CMY_8_PLANAR; color_space CmsColorSpace::Cmy, channels 3, bps 1, planar);
    PixelTypeDef!(pub const CMY_16; color_space CmsColorSpace::Cmy, channels 3, bps 2);
    PixelTypeDef!(pub const CMY_16_PLANAR; color_space CmsColorSpace::Cmy, channels 3, bps 2, planar);
    PixelTypeDef!(pub const CMY_16_SE; color_space CmsColorSpace::Cmy, channels 3, bps 2, endian16);
    PixelTypeDef!(pub const CMYK_8; color_space CmsColorSpace::Cmyk, channels 4, bps 1);
    PixelTypeDef!(pub const CMYKA_8; color_space CmsColorSpace::Cmyk, extra 1, channels 4, bps 1);
    PixelTypeDef!(pub const CMYK_8_REV; color_space CmsColorSpace::Cmyk, channels 4, bps 1, min_is_white);
    pub const YUVK_8: CmsPixelType = Self::CMYK_8_REV;
    PixelTypeDef!(pub const CMYK_8_PLANAR; color_space CmsColorSpace::Cmyk, channels 4, bps 1, planar);
    PixelTypeDef!(pub const CMYK_16; color_space CmsColorSpace::Cmyk, channels 4, bps 2);
    PixelTypeDef!(pub const CMYK_16_REV; color_space CmsColorSpace::Cmyk, channels 4, bps 2, min_is_white);
    pub const YUVK_16: CmsPixelType = Self::CMYK_16_REV;
    PixelTypeDef!(pub const CMYK_16_PLANAR; color_space CmsColorSpace::Cmyk, channels 4, bps 2, planar);
    PixelTypeDef!(pub const CMYK_16_SE; color_space CmsColorSpace::Cmyk, channels 4, bps 2, endian16);
    PixelTypeDef!(pub const KYMC_8; color_space CmsColorSpace::Cmyk, channels 4, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC_16; color_space CmsColorSpace::Cmyk, channels 4, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC_16_SE; color_space CmsColorSpace::Cmyk, channels 4, bps 2, do_swap, endian16);
    PixelTypeDef!(pub const KCMY_8; color_space CmsColorSpace::Cmyk, channels 4, bps 1, swap_first);
    PixelTypeDef!(pub const KCMY_8_REV; color_space CmsColorSpace::Cmyk, channels 4, bps 1, min_is_white, swap_first);
    PixelTypeDef!(pub const KCMY_16; color_space CmsColorSpace::Cmyk, channels 4, bps 2, swap_first);
    PixelTypeDef!(pub const KCMY_16_REV; color_space CmsColorSpace::Cmyk, channels 4, bps 2, min_is_white, swap_first);
    PixelTypeDef!(pub const KCMY_16_SE; color_space CmsColorSpace::Cmyk, channels 4, bps 2, endian16, swap_first);
    PixelTypeDef!(pub const CMYK5_8; color_space CmsColorSpace::Mch5, channels 5, bps 1);
    PixelTypeDef!(pub const CMYK5_16; color_space CmsColorSpace::Mch5, channels 5, bps 2);
    PixelTypeDef!(pub const CMYK5_16_SE; color_space CmsColorSpace::Mch5, channels 5, bps 2, endian16);
    PixelTypeDef!(pub const KYMC5_8; color_space CmsColorSpace::Mch5, channels 5, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC5_16; color_space CmsColorSpace::Mch5, channels 5, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC5_16_SE; color_space CmsColorSpace::Mch5, channels 5, bps 2, do_swap, endian16);
    PixelTypeDef!(pub const CMYK6_8; color_space CmsColorSpace::Mch6, channels 6, bps 1);
    PixelTypeDef!(pub const CMYK6_8_PLANAR; color_space CmsColorSpace::Mch6, channels 6, bps 1, planar);
    PixelTypeDef!(pub const CMYK6_16; color_space CmsColorSpace::Mch6, channels 6, bps 2);
    PixelTypeDef!(pub const CMYK6_16_PLANAR; color_space CmsColorSpace::Mch6, channels 6, bps 2, planar);
    PixelTypeDef!(pub const CMYK6_16_SE; color_space CmsColorSpace::Mch6, channels 6, bps 2, endian16);
    PixelTypeDef!(pub const CMYK7_8; color_space CmsColorSpace::Mch7, channels 7, bps 1);
    PixelTypeDef!(pub const CMYK7_16; color_space CmsColorSpace::Mch7, channels 7, bps 2);
    PixelTypeDef!(pub const CMYK7_16_SE; color_space CmsColorSpace::Mch7, channels 7, bps 2, endian16);
    PixelTypeDef!(pub const KYMC7_8; color_space CmsColorSpace::Mch7, channels 7, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC7_16; color_space CmsColorSpace::Mch7, channels 7, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC7_16_SE; color_space CmsColorSpace::Mch7, channels 7, bps 2, do_swap, endian16);
    PixelTypeDef!(pub const CMYK8_8; color_space CmsColorSpace::Mch8, channels 8, bps 1);
    PixelTypeDef!(pub const CMYK8_16; color_space CmsColorSpace::Mch8, channels 8, bps 2);
    PixelTypeDef!(pub const CMYK8_16_SE; color_space CmsColorSpace::Mch8, channels 8, bps 2, endian16);
    PixelTypeDef!(pub const KYMC8_8; color_space CmsColorSpace::Mch8, channels 8, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC8_16; color_space CmsColorSpace::Mch8, channels 8, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC8_16_SE; color_space CmsColorSpace::Mch8, channels 8, bps 2, do_swap, endian16);
    PixelTypeDef!(pub const CMYK9_8; color_space CmsColorSpace::Mch9, channels 9, bps 1);
    PixelTypeDef!(pub const CMYK9_16; color_space CmsColorSpace::Mch9, channels 9, bps 2);
    PixelTypeDef!(pub const CMYK9_16_SE; color_space CmsColorSpace::Mch9, channels 9, bps 2, endian16);
    PixelTypeDef!(pub const KYMC9_8; color_space CmsColorSpace::Mch9, channels 9, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC9_16; color_space CmsColorSpace::Mch9, channels 9, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC9_16_SE; color_space CmsColorSpace::Mch9, channels 9, bps 2, do_swap, endian16);
    PixelTypeDef!(pub const CMYK10_8; color_space CmsColorSpace::Mch10, channels 10, bps 1);
    PixelTypeDef!(pub const CMYK10_16; color_space CmsColorSpace::Mch10, channels 10, bps 2);
    PixelTypeDef!(pub const CMYK10_16_SE; color_space CmsColorSpace::Mch10, channels 10, bps 2, endian16);
    PixelTypeDef!(pub const KYMC10_8; color_space CmsColorSpace::Mch10, channels 10, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC10_16; color_space CmsColorSpace::Mch10, channels 10, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC10_16_SE; color_space CmsColorSpace::Mch10, channels 10, bps 2, do_swap, endian16);
    PixelTypeDef!(pub const CMYK11_8; color_space CmsColorSpace::Mch11, channels 11, bps 1);
    PixelTypeDef!(pub const CMYK11_16; color_space CmsColorSpace::Mch11, channels 11, bps 2);
    PixelTypeDef!(pub const CMYK11_16_SE; color_space CmsColorSpace::Mch11, channels 11, bps 2, endian16);
    PixelTypeDef!(pub const KYMC11_8; color_space CmsColorSpace::Mch11, channels 11, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC11_16; color_space CmsColorSpace::Mch11, channels 11, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC11_16_SE; color_space CmsColorSpace::Mch11, channels 11, bps 2, do_swap, endian16);
    PixelTypeDef!(pub const CMYK12_8; color_space CmsColorSpace::Mch12, channels 12, bps 1);
    PixelTypeDef!(pub const CMYK12_16; color_space CmsColorSpace::Mch12, channels 12, bps 2);
    PixelTypeDef!(pub const CMYK12_16_SE; color_space CmsColorSpace::Mch12, channels 12, bps 2, endian16);
    PixelTypeDef!(pub const KYMC12_8; color_space CmsColorSpace::Mch12, channels 12, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC12_16; color_space CmsColorSpace::Mch12, channels 12, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC12_16_SE; color_space CmsColorSpace::Mch12, channels 12, bps 2, do_swap, endian16);
    // Colorimetric
    PixelTypeDef!(pub const XYZ_16; color_space CmsColorSpace::Xyz, channels 3, bps 2);
    PixelTypeDef!(pub const LAB_8; color_space CmsColorSpace::Lab, channels 3, bps 1);
    PixelTypeDef!(pub const LAB_V2_8; color_space CmsColorSpace::LabV2, channels 3, bps 1);
    PixelTypeDef!(pub const ALAB_8; color_space CmsColorSpace::Lab, channels 3, bps 1, extra 1, swap_first);
    PixelTypeDef!(pub const ALAB_V2_8; color_space CmsColorSpace::LabV2, channels 3, bps 1, extra 1, swap_first);
    PixelTypeDef!(pub const LAB_16; color_space CmsColorSpace::Lab, channels 3, bps 2);
    PixelTypeDef!(pub const LAB_V2_16; color_space CmsColorSpace::LabV2, channels 3, bps 2);
    PixelTypeDef!(pub const YXY_16; color_space CmsColorSpace::Yxy, channels 3, bps 2);
    // YCbCr
    PixelTypeDef!(pub const YCBCR_8; color_space CmsColorSpace::YCbCr, channels 3, bps 1);
    PixelTypeDef!(pub const YCBCR_8_PLANAR; color_space CmsColorSpace::YCbCr, channels 3, bps 1, planar);
    PixelTypeDef!(pub const YCBCR_16; color_space CmsColorSpace::YCbCr, channels 3, bps 2);
    PixelTypeDef!(pub const YCBCR_16_PLANAR; color_space CmsColorSpace::YCbCr, channels 3, bps 2, planar);
    PixelTypeDef!(pub const YCBCR_16_SE; color_space CmsColorSpace::YCbCr, channels 3, bps 2, endian16);
    // YUV
    PixelTypeDef!(pub const YUV_8; color_space CmsColorSpace::Yuv, channels 3, bps 1);
    PixelTypeDef!(pub const YUV_8_PLANAR; color_space CmsColorSpace::Yuv, channels 3, bps 1, planar);
    PixelTypeDef!(pub const YUV_16; color_space CmsColorSpace::Yuv, channels 3, bps 2);
    PixelTypeDef!(pub const YUV_16_PLANAR; color_space CmsColorSpace::Yuv, channels 3, bps 2, planar);
    PixelTypeDef!(pub const YUV_16_SE; color_space CmsColorSpace::Yuv, channels 3, bps 2, endian16);
    // HLS
    PixelTypeDef!(pub const HLS_8; color_space CmsColorSpace::Hls, channels 3, bps 1);
    PixelTypeDef!(pub const HLS_8_PLANAR; color_space CmsColorSpace::Hls, channels 3, bps 1, planar);
    PixelTypeDef!(pub const HLS_16; color_space CmsColorSpace::Hls, channels 3, bps 2);
    PixelTypeDef!(pub const HLS_16_PLANAR; color_space CmsColorSpace::Hls, channels 3, bps 2, planar);
    PixelTypeDef!(pub const HLS_16_SE; color_space CmsColorSpace::Hls, channels 3, bps 2, endian16);
    // HSV
    PixelTypeDef!(pub const HSV_8; color_space CmsColorSpace::Hsv, channels 3, bps 1);
    PixelTypeDef!(pub const HSV_8_PLANAR; color_space CmsColorSpace::Hsv, channels 3, bps 1, planar);
    PixelTypeDef!(pub const HSV_16; color_space CmsColorSpace::Hsv, channels 3, bps 2);
    PixelTypeDef!(pub const HSV_16_PLANAR; color_space CmsColorSpace::Hsv, channels 3, bps 2, planar);
    PixelTypeDef!(pub const HSV_16_SE; color_space CmsColorSpace::Hsv, channels 3, bps 2, endian16);
    // Named color index. Only 16 bits allowed (don't check colorspace)
    PixelTypeDef!(pub const NAMED_COLOR_INDEX; channels 1, bps 2);
    // Float formatters.
    PixelTypeDef!(pub const XYZ_FLT; float, color_space CmsColorSpace::Xyz, channels 3, bps 4);
    PixelTypeDef!(pub const LAB_FLT; float, color_space CmsColorSpace::Lab, channels 3, bps 4);
    PixelTypeDef!(pub const LABA_FLT; float, color_space CmsColorSpace::Lab, extra 1, channels 3, bps 4);
    PixelTypeDef!(pub const GRAY_FLT; float, color_space CmsColorSpace::Gray, channels 1, bps 4);
    PixelTypeDef!(pub const RGB_FLT; float, color_space CmsColorSpace::Rgb, channels 3, bps 4);
    PixelTypeDef!(pub const RGBA_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4);
    PixelTypeDef!(pub const ARGB_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, swap_first);
    PixelTypeDef!(pub const BGR_FLT; float, color_space CmsColorSpace::Rgb, channels 3, bps 4, do_swap);
    PixelTypeDef!(pub const BGRA_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap, swap_first);
    PixelTypeDef!(pub const ABGR_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap);
    PixelTypeDef!(pub const CMYK_FLT; float, color_space CmsColorSpace::Cmyk, channels 4, bps 4);
    // Floating point formatters.
    // NOTE THAT 'BYTES' FIELD IS SET TO ZERO ON DLB because 8 bytes overflows the bitfield
    PixelTypeDef!(pub const XYZ_DBL; float, color_space CmsColorSpace::Xyz, channels 3, bps 0);
    PixelTypeDef!(pub const LAB_DBL; float, color_space CmsColorSpace::Lab, channels 3, bps 0);
    PixelTypeDef!(pub const GRAY_DBL; float, color_space CmsColorSpace::Gray, channels 1, bps 0);
    PixelTypeDef!(pub const RGB_DBL; float, color_space CmsColorSpace::Rgb, channels 3, bps 0);
    PixelTypeDef!(pub const BGR_DBL; float, color_space CmsColorSpace::Rgb, channels 3, bps 0, do_swap);
    PixelTypeDef!(pub const CMYK_DBL; float, color_space CmsColorSpace::Cmyk, channels 4, bps 0);
    // IEEE 754-2008 "half"
    PixelTypeDef!(pub const GRAY_HALF_FLT; float, color_space CmsColorSpace::Gray, channels 1, bps 2);
    PixelTypeDef!(pub const RGB_HALF_FLT; float, color_space CmsColorSpace::Rgb, channels 3, bps 2);
    PixelTypeDef!(pub const RGBA_HALF_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2);
    PixelTypeDef!(pub const CMYK_HALF_FLT; float, color_space CmsColorSpace::Cmyk, channels 4, bps 2);
    PixelTypeDef!(pub const ARGB_HALF_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, swap_first);
    PixelTypeDef!(pub const BGR_HALF_FLT; float, color_space CmsColorSpace::Rgb, channels 3, bps 2, do_swap);
    PixelTypeDef!(pub const BGRA_HALF_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, swap_first);
    PixelTypeDef!(pub const ABGR_HALF_FLT; float, color_space CmsColorSpace::Rgb, channels 3, bps 2, do_swap);

    pub fn set_color_space(&mut self, cs: CmsColorSpace) {
        self._set_color_space(cs as u8);
    }

    pub fn color_space(self) -> CmsColorSpace {
        match self._color_space() {
            3 => CmsColorSpace::Gray,
            4 => CmsColorSpace::Rgb,
            5 => CmsColorSpace::Cmy,
            6 => CmsColorSpace::Cmyk,
            7 => CmsColorSpace::YCbCr,
            8 => CmsColorSpace::Yuv,
            9 => CmsColorSpace::Xyz,
            10 => CmsColorSpace::Lab,
            11 => CmsColorSpace::Yuvk,
            12 => CmsColorSpace::Hsv,
            13 => CmsColorSpace::Hls,
            14 => CmsColorSpace::Yxy,
            15 => CmsColorSpace::Mch1,
            16 => CmsColorSpace::Mch2,
            17 => CmsColorSpace::Mch3,
            18 => CmsColorSpace::Mch4,
            19 => CmsColorSpace::Mch5,
            20 => CmsColorSpace::Mch6,
            21 => CmsColorSpace::Mch7,
            22 => CmsColorSpace::Mch8,
            23 => CmsColorSpace::Mch9,
            24 => CmsColorSpace::Mch10,
            25 => CmsColorSpace::Mch11,
            26 => CmsColorSpace::Mch12,
            27 => CmsColorSpace::Mch13,
            28 => CmsColorSpace::Mch14,
            29 => CmsColorSpace::Mch15,
            30 => CmsColorSpace::LabV2,
            _ => CmsColorSpace::Any,
        }
    }
}

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

pub mod plugin;
mod internal;
