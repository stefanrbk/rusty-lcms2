use super::ColorSpace as ColorSpace;

use bitfield::bitfield;

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
    pub struct PixelType(u32);
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
macro_rules! PixelTypeDef {
    ($($name:ident)*; $($rest:tt)*) => {
        $($name)*: PixelType = PixelType!($($rest)*);
    };
}

macro_rules! PixelType {
    (set) => (0);
    (set premul) => (
        (1 << 23)
    );
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
        PixelType { 0: PixelType!(set $head $($val)?)}
    );
    ($head:ident $($val:expr)?, $($tail1:ident $($tail2:expr)?),*) => (
        PixelType { 0: PixelType!(set $head $($val)?, $($tail1 $($tail2)?),*)}
    );
}

impl PixelType {
    PixelTypeDef!(pub const GRAY_8; color_space ColorSpace::Gray, channels 1, bps 1);
    PixelTypeDef!(pub const GRAY_8_REV; color_space ColorSpace::Gray, channels 1, bps 1, min_is_white);
    PixelTypeDef!(pub const GRAY_16; color_space ColorSpace::Gray,channels 1, bps 2);
    PixelTypeDef!(pub const GRAY_16_REV; color_space ColorSpace::Gray,channels 1, bps 2, min_is_white);
    PixelTypeDef!(pub const GRAY_16_SE; color_space ColorSpace::Gray,channels 1, bps 2, endian16);
    PixelTypeDef!(pub const GRAYA_8; color_space ColorSpace::Gray, extra 1, channels 1, bps 1);
    PixelTypeDef!(pub const GRAYA_8_PREMUL; color_space ColorSpace::Gray, extra 1, channels 1, bps 1, premul);
    PixelTypeDef!(pub const GRAYA_16; color_space ColorSpace::Gray, extra 1, channels 1, bps 2);
    PixelTypeDef!(pub const GRAYA_16_PREMUL; color_space ColorSpace::Gray, extra 1, channels 1, bps 2, premul);
    PixelTypeDef!(pub const GRAYA_16_SE; color_space ColorSpace::Gray, extra 1, channels 1, bps 2, endian16);
    PixelTypeDef!(pub const GRAYA_8_PLANAR; color_space ColorSpace::Gray, extra 1, channels 1, bps 1, planar);
    PixelTypeDef!(pub const GRAYA_16_PLANAR; color_space ColorSpace::Gray, extra 1, channels 1, bps 2, planar);

    PixelTypeDef!(pub const RGB_8; color_space ColorSpace::Rgb, channels 3, bps 1);
    PixelTypeDef!(pub const RGB_8_PLANAR; color_space ColorSpace::Rgb, channels 3, bps 1, planar);
    PixelTypeDef!(pub const BGR_8; color_space ColorSpace::Rgb, channels 3, bps 1, do_swap);
    PixelTypeDef!(pub const BGR_8_PLANAR; color_space ColorSpace::Rgb, channels 3, bps 1, do_swap, planar);
    PixelTypeDef!(pub const RGB_16; color_space ColorSpace::Rgb, channels 3, bps 2);
    PixelTypeDef!(pub const RGB_16_PLANAR; color_space ColorSpace::Rgb, channels 3, bps 2, planar);
    PixelTypeDef!(pub const RGB_16_SE; color_space ColorSpace::Rgb, channels 3, bps 2, endian16);
    PixelTypeDef!(pub const BGR_16; color_space ColorSpace::Rgb, channels 3, bps 2, do_swap);
    PixelTypeDef!(pub const BGR_16_PLANAR; color_space ColorSpace::Rgb, channels 3, bps 2, do_swap, planar);
    PixelTypeDef!(pub const BGR_16_SE; color_space ColorSpace::Rgb, channels 3, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const RGBA_8; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1);
    PixelTypeDef!(pub const RGBA_8_PREMUL; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, premul);
    PixelTypeDef!(pub const RGBA_8_PLANAR; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, planar);
    PixelTypeDef!(pub const RGBA_16; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2);
    PixelTypeDef!(pub const RGBA_16_PREMUL; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, premul);
    PixelTypeDef!(pub const RGBA_16_PLANAR; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, planar);
    PixelTypeDef!(pub const RGBA_16_SE; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, endian16);

    PixelTypeDef!(pub const ARGB_8; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, swap_first);
    PixelTypeDef!(pub const ARGB_8_PREMUL; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, swap_first, premul);
    PixelTypeDef!(pub const ARGB_8_PLANAR; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, swap_first, planar);
    PixelTypeDef!(pub const ARGB_16; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, swap_first);
    PixelTypeDef!(pub const ARGB_16_PREMUL; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, swap_first, premul);

    PixelTypeDef!(pub const ABGR_8; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap);
    PixelTypeDef!(pub const ABGR_8_PREMUL; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, premul);
    PixelTypeDef!(pub const ABGR_8_PLANAR; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, planar);
    PixelTypeDef!(pub const ABGR_16; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap);
    PixelTypeDef!(pub const ABGR_16_PREMUL; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, premul);
    PixelTypeDef!(pub const ABGR_16_PLANAR; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, planar);
    PixelTypeDef!(pub const ABGR_16_SE; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const BGRA_8; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, swap_first);
    PixelTypeDef!(pub const BGRA_8_PREMUL; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, swap_first, premul);
    PixelTypeDef!(pub const BGRA_8_PLANAR; color_space ColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, swap_first, planar);
    PixelTypeDef!(pub const BGRA_16; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, swap_first);
    PixelTypeDef!(pub const BGRA_16_PREMUL; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, swap_first, premul);
    PixelTypeDef!(pub const BGRA_16_SE; color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, endian16, do_swap, swap_first);
    
    PixelTypeDef!(pub const CMY_8; color_space ColorSpace::Cmy, channels 3, bps 1);
    PixelTypeDef!(pub const CMY_8_PLANAR; color_space ColorSpace::Cmy, channels 3, bps 1, planar);
    PixelTypeDef!(pub const CMY_16; color_space ColorSpace::Cmy, channels 3, bps 2);
    PixelTypeDef!(pub const CMY_16_PLANAR; color_space ColorSpace::Cmy, channels 3, bps 2, planar);
    PixelTypeDef!(pub const CMY_16_SE; color_space ColorSpace::Cmy, channels 3, bps 2, endian16);

    PixelTypeDef!(pub const CMYK_8; color_space ColorSpace::Cmyk, channels 4, bps 1);
    PixelTypeDef!(pub const CMYKA_8; color_space ColorSpace::Cmyk, extra 1, channels 4, bps 1);
    PixelTypeDef!(pub const CMYK_8_REV; color_space ColorSpace::Cmyk, channels 4, bps 1, min_is_white);
    pub const YUVK_8: PixelType = Self::CMYK_8_REV;
    PixelTypeDef!(pub const CMYK_8_PLANAR; color_space ColorSpace::Cmyk, channels 4, bps 1, planar);
    PixelTypeDef!(pub const CMYK_16; color_space ColorSpace::Cmyk, channels 4, bps 2);
    PixelTypeDef!(pub const CMYK_16_REV; color_space ColorSpace::Cmyk, channels 4, bps 2, min_is_white);
    pub const YUVK_16: PixelType = Self::CMYK_16_REV;
    PixelTypeDef!(pub const CMYK_16_PLANAR; color_space ColorSpace::Cmyk, channels 4, bps 2, planar);
    PixelTypeDef!(pub const CMYK_16_SE; color_space ColorSpace::Cmyk, channels 4, bps 2, endian16);

    PixelTypeDef!(pub const KYMC_8; color_space ColorSpace::Cmyk, channels 4, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC_16; color_space ColorSpace::Cmyk, channels 4, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC_16_SE; color_space ColorSpace::Cmyk, channels 4, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const KCMY_8; color_space ColorSpace::Cmyk, channels 4, bps 1, swap_first);
    PixelTypeDef!(pub const KCMY_8_REV; color_space ColorSpace::Cmyk, channels 4, bps 1, min_is_white, swap_first);
    PixelTypeDef!(pub const KCMY_16; color_space ColorSpace::Cmyk, channels 4, bps 2, swap_first);
    PixelTypeDef!(pub const KCMY_16_REV; color_space ColorSpace::Cmyk, channels 4, bps 2, min_is_white, swap_first);
    PixelTypeDef!(pub const KCMY_16_SE; color_space ColorSpace::Cmyk, channels 4, bps 2, endian16, swap_first);

    PixelTypeDef!(pub const CMYK5_8; color_space ColorSpace::Mch5, channels 5, bps 1);
    PixelTypeDef!(pub const CMYK5_16; color_space ColorSpace::Mch5, channels 5, bps 2);
    PixelTypeDef!(pub const CMYK5_16_SE; color_space ColorSpace::Mch5, channels 5, bps 2, endian16);
    PixelTypeDef!(pub const KYMC5_8; color_space ColorSpace::Mch5, channels 5, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC5_16; color_space ColorSpace::Mch5, channels 5, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC5_16_SE; color_space ColorSpace::Mch5, channels 5, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const CMYK6_8; color_space ColorSpace::Mch6, channels 6, bps 1);
    PixelTypeDef!(pub const CMYK6_8_PLANAR; color_space ColorSpace::Mch6, channels 6, bps 1, planar);
    PixelTypeDef!(pub const CMYK6_16; color_space ColorSpace::Mch6, channels 6, bps 2);
    PixelTypeDef!(pub const CMYK6_16_PLANAR; color_space ColorSpace::Mch6, channels 6, bps 2, planar);
    PixelTypeDef!(pub const CMYK6_16_SE; color_space ColorSpace::Mch6, channels 6, bps 2, endian16);

    PixelTypeDef!(pub const CMYK7_8; color_space ColorSpace::Mch7, channels 7, bps 1);
    PixelTypeDef!(pub const CMYK7_16; color_space ColorSpace::Mch7, channels 7, bps 2);
    PixelTypeDef!(pub const CMYK7_16_SE; color_space ColorSpace::Mch7, channels 7, bps 2, endian16);
    PixelTypeDef!(pub const KYMC7_8; color_space ColorSpace::Mch7, channels 7, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC7_16; color_space ColorSpace::Mch7, channels 7, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC7_16_SE; color_space ColorSpace::Mch7, channels 7, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const CMYK8_8; color_space ColorSpace::Mch8, channels 8, bps 1);
    PixelTypeDef!(pub const CMYK8_16; color_space ColorSpace::Mch8, channels 8, bps 2);
    PixelTypeDef!(pub const CMYK8_16_SE; color_space ColorSpace::Mch8, channels 8, bps 2, endian16);
    PixelTypeDef!(pub const KYMC8_8; color_space ColorSpace::Mch8, channels 8, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC8_16; color_space ColorSpace::Mch8, channels 8, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC8_16_SE; color_space ColorSpace::Mch8, channels 8, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const CMYK9_8; color_space ColorSpace::Mch9, channels 9, bps 1);
    PixelTypeDef!(pub const CMYK9_16; color_space ColorSpace::Mch9, channels 9, bps 2);
    PixelTypeDef!(pub const CMYK9_16_SE; color_space ColorSpace::Mch9, channels 9, bps 2, endian16);
    PixelTypeDef!(pub const KYMC9_8; color_space ColorSpace::Mch9, channels 9, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC9_16; color_space ColorSpace::Mch9, channels 9, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC9_16_SE; color_space ColorSpace::Mch9, channels 9, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const CMYK10_8; color_space ColorSpace::Mch10, channels 10, bps 1);
    PixelTypeDef!(pub const CMYK10_16; color_space ColorSpace::Mch10, channels 10, bps 2);
    PixelTypeDef!(pub const CMYK10_16_SE; color_space ColorSpace::Mch10, channels 10, bps 2, endian16);
    PixelTypeDef!(pub const KYMC10_8; color_space ColorSpace::Mch10, channels 10, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC10_16; color_space ColorSpace::Mch10, channels 10, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC10_16_SE; color_space ColorSpace::Mch10, channels 10, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const CMYK11_8; color_space ColorSpace::Mch11, channels 11, bps 1);
    PixelTypeDef!(pub const CMYK11_16; color_space ColorSpace::Mch11, channels 11, bps 2);
    PixelTypeDef!(pub const CMYK11_16_SE; color_space ColorSpace::Mch11, channels 11, bps 2, endian16);
    PixelTypeDef!(pub const KYMC11_8; color_space ColorSpace::Mch11, channels 11, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC11_16; color_space ColorSpace::Mch11, channels 11, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC11_16_SE; color_space ColorSpace::Mch11, channels 11, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const CMYK12_8; color_space ColorSpace::Mch12, channels 12, bps 1);
    PixelTypeDef!(pub const CMYK12_16; color_space ColorSpace::Mch12, channels 12, bps 2);
    PixelTypeDef!(pub const CMYK12_16_SE; color_space ColorSpace::Mch12, channels 12, bps 2, endian16);
    PixelTypeDef!(pub const KYMC12_8; color_space ColorSpace::Mch12, channels 12, bps 1, do_swap);
    PixelTypeDef!(pub const KYMC12_16; color_space ColorSpace::Mch12, channels 12, bps 2, do_swap);
    PixelTypeDef!(pub const KYMC12_16_SE; color_space ColorSpace::Mch12, channels 12, bps 2, do_swap, endian16);

    // Colorimetric
    PixelTypeDef!(pub const XYZ_16; color_space ColorSpace::Xyz, channels 3, bps 2);
    PixelTypeDef!(pub const LAB_8; color_space ColorSpace::Lab, channels 3, bps 1);
    PixelTypeDef!(pub const LAB_V2_8; color_space ColorSpace::LabV2, channels 3, bps 1);

    PixelTypeDef!(pub const ALAB_8; color_space ColorSpace::Lab, channels 3, bps 1, extra 1, swap_first);
    PixelTypeDef!(pub const ALAB_V2_8; color_space ColorSpace::LabV2, channels 3, bps 1, extra 1, swap_first);
    PixelTypeDef!(pub const LAB_16; color_space ColorSpace::Lab, channels 3, bps 2);
    PixelTypeDef!(pub const LAB_V2_16; color_space ColorSpace::LabV2, channels 3, bps 2);
    PixelTypeDef!(pub const YXY_16; color_space ColorSpace::Yxy, channels 3, bps 2);

    // YCbCr
    PixelTypeDef!(pub const YCBCR_8; color_space ColorSpace::YCbCr, channels 3, bps 1);
    PixelTypeDef!(pub const YCBCR_8_PLANAR; color_space ColorSpace::YCbCr, channels 3, bps 1, planar);
    PixelTypeDef!(pub const YCBCR_16; color_space ColorSpace::YCbCr, channels 3, bps 2);
    PixelTypeDef!(pub const YCBCR_16_PLANAR; color_space ColorSpace::YCbCr, channels 3, bps 2, planar);
    PixelTypeDef!(pub const YCBCR_16_SE; color_space ColorSpace::YCbCr, channels 3, bps 2, endian16);

    // YUV
    PixelTypeDef!(pub const YUV_8; color_space ColorSpace::Yuv, channels 3, bps 1);
    PixelTypeDef!(pub const YUV_8_PLANAR; color_space ColorSpace::Yuv, channels 3, bps 1, planar);
    PixelTypeDef!(pub const YUV_16; color_space ColorSpace::Yuv, channels 3, bps 2);
    PixelTypeDef!(pub const YUV_16_PLANAR; color_space ColorSpace::Yuv, channels 3, bps 2, planar);
    PixelTypeDef!(pub const YUV_16_SE; color_space ColorSpace::Yuv, channels 3, bps 2, endian16);

    // HLS
    PixelTypeDef!(pub const HLS_8; color_space ColorSpace::Hls, channels 3, bps 1);
    PixelTypeDef!(pub const HLS_8_PLANAR; color_space ColorSpace::Hls, channels 3, bps 1, planar);
    PixelTypeDef!(pub const HLS_16; color_space ColorSpace::Hls, channels 3, bps 2);
    PixelTypeDef!(pub const HLS_16_PLANAR; color_space ColorSpace::Hls, channels 3, bps 2, planar);
    PixelTypeDef!(pub const HLS_16_SE; color_space ColorSpace::Hls, channels 3, bps 2, endian16);

    // HSV
    PixelTypeDef!(pub const HSV_8; color_space ColorSpace::Hsv, channels 3, bps 1);
    PixelTypeDef!(pub const HSV_8_PLANAR; color_space ColorSpace::Hsv, channels 3, bps 1, planar);
    PixelTypeDef!(pub const HSV_16; color_space ColorSpace::Hsv, channels 3, bps 2);
    PixelTypeDef!(pub const HSV_16_PLANAR; color_space ColorSpace::Hsv, channels 3, bps 2, planar);
    PixelTypeDef!(pub const HSV_16_SE; color_space ColorSpace::Hsv, channels 3, bps 2, endian16);

    // Named color index. Only 16 bits allowed (don't check colorspace)
    PixelTypeDef!(pub const NAMED_COLOR_INDEX; channels 1, bps 2);

    // Float formatters.
    PixelTypeDef!(pub const XYZ_FLT; float, color_space ColorSpace::Xyz, channels 3, bps 4);
    PixelTypeDef!(pub const LAB_FLT; float, color_space ColorSpace::Lab, channels 3, bps 4);
    PixelTypeDef!(pub const LABA_FLT; float, color_space ColorSpace::Lab, extra 1, channels 3, bps 4);
    PixelTypeDef!(pub const GRAY_FLT; float, color_space ColorSpace::Gray, channels 1, bps 4);
    PixelTypeDef!(pub const GRAYA_FLT; float, color_space ColorSpace::Gray, channels 1, bps 4, extra 1);
    PixelTypeDef!(pub const GRAYA_FLT_PREMUL; float, color_space ColorSpace::Gray, channels 1, bps 4, extra 1, premul);
    PixelTypeDef!(pub const RGB_FLT; float, color_space ColorSpace::Rgb, channels 3, bps 4);

    PixelTypeDef!(pub const RGBA_FLT; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 4);
    PixelTypeDef!(pub const RGBA_FLT_PREMUL; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 4, premul);
    PixelTypeDef!(pub const ARGB_FLT; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 4, swap_first);
    PixelTypeDef!(pub const ARGB_FLT_PREMUL; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 4, swap_first, premul);
    PixelTypeDef!(pub const BGR_FLT; float, color_space ColorSpace::Rgb, channels 3, bps 4, do_swap);
    PixelTypeDef!(pub const BGRA_FLT; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap, swap_first);
    PixelTypeDef!(pub const BGRA_FLT_PREMUL; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap, swap_first, premul);
    PixelTypeDef!(pub const ABGR_FLT; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap);
    PixelTypeDef!(pub const ABGR_FLT_PREMUL; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap, premul);

    PixelTypeDef!(pub const CMYK_FLT; float, color_space ColorSpace::Cmyk, channels 4, bps 4);

    // Floating point formatters.
    // NOTE THAT 'BYTES' FIELD IS SET TO ZERO ON DLB because 8 bytes overflows the bitfield
    PixelTypeDef!(pub const XYZ_DBL; float, color_space ColorSpace::Xyz, channels 3, bps 0);
    PixelTypeDef!(pub const LAB_DBL; float, color_space ColorSpace::Lab, channels 3, bps 0);
    PixelTypeDef!(pub const GRAY_DBL; float, color_space ColorSpace::Gray, channels 1, bps 0);
    PixelTypeDef!(pub const RGB_DBL; float, color_space ColorSpace::Rgb, channels 3, bps 0);
    PixelTypeDef!(pub const BGR_DBL; float, color_space ColorSpace::Rgb, channels 3, bps 0, do_swap);
    PixelTypeDef!(pub const CMYK_DBL; float, color_space ColorSpace::Cmyk, channels 4, bps 0);

    // IEEE 754-2008 "half"
    PixelTypeDef!(pub const GRAY_HALF_FLT; float, color_space ColorSpace::Gray, channels 1, bps 2);
    PixelTypeDef!(pub const RGB_HALF_FLT; float, color_space ColorSpace::Rgb, channels 3, bps 2);
    PixelTypeDef!(pub const RGBA_HALF_FLT; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 2);
    PixelTypeDef!(pub const CMYK_HALF_FLT; float, color_space ColorSpace::Cmyk, channels 4, bps 2);
    
    PixelTypeDef!(pub const ARGB_HALF_FLT; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, swap_first);
    PixelTypeDef!(pub const BGR_HALF_FLT; float, color_space ColorSpace::Rgb, channels 3, bps 2, do_swap);
    PixelTypeDef!(pub const BGRA_HALF_FLT; float, color_space ColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, swap_first);
    PixelTypeDef!(pub const ABGR_HALF_FLT; float, color_space ColorSpace::Rgb, channels 3, bps 2, do_swap);

    pub fn set_color_space(&mut self, cs: ColorSpace) {
        self._set_color_space(cs as u8);
    }

    pub fn color_space(self) -> ColorSpace {
        match self._color_space() {
            3 => ColorSpace::Gray,
            4 => ColorSpace::Rgb,
            5 => ColorSpace::Cmy,
            6 => ColorSpace::Cmyk,
            7 => ColorSpace::YCbCr,
            8 => ColorSpace::Yuv,
            9 => ColorSpace::Xyz,
            10 => ColorSpace::Lab,
            11 => ColorSpace::Yuvk,
            12 => ColorSpace::Hsv,
            13 => ColorSpace::Hls,
            14 => ColorSpace::Yxy,
            15 => ColorSpace::Mch1,
            16 => ColorSpace::Mch2,
            17 => ColorSpace::Mch3,
            18 => ColorSpace::Mch4,
            19 => ColorSpace::Mch5,
            20 => ColorSpace::Mch6,
            21 => ColorSpace::Mch7,
            22 => ColorSpace::Mch8,
            23 => ColorSpace::Mch9,
            24 => ColorSpace::Mch10,
            25 => ColorSpace::Mch11,
            26 => ColorSpace::Mch12,
            27 => ColorSpace::Mch13,
            28 => ColorSpace::Mch14,
            29 => ColorSpace::Mch15,
            30 => ColorSpace::LabV2,
            _ => ColorSpace::Any,
        }
    }
}
