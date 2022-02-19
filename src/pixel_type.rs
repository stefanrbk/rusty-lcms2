use super::CmsPixelType as CmsPixelType;
use super::CmsColorSpace as CmsColorSpace;

macro_rules! PixelTypeDef {
    ($($name:ident)*; $($rest:tt)*) => {
        $($name)*: CmsPixelType = PixelType!($($rest)*);
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
        CmsPixelType { 0: PixelType!(set $head $($val)?)}
    );
    ($head:ident $($val:expr)?, $($tail1:ident $($tail2:expr)?),*) => (
        CmsPixelType { 0: PixelType!(set $head $($val)?, $($tail1 $($tail2)?),*)}
    );
}

impl CmsPixelType {
    PixelTypeDef!(pub const GRAY_8; color_space CmsColorSpace::Gray, channels 1, bps 1);
    PixelTypeDef!(pub const GRAY_8_REV; color_space CmsColorSpace::Gray, channels 1, bps 1, min_is_white);
    PixelTypeDef!(pub const GRAY_16; color_space CmsColorSpace::Gray,channels 1, bps 2);
    PixelTypeDef!(pub const GRAY_16_REV; color_space CmsColorSpace::Gray,channels 1, bps 2, min_is_white);
    PixelTypeDef!(pub const GRAY_16_SE; color_space CmsColorSpace::Gray,channels 1, bps 2, endian16);
    PixelTypeDef!(pub const GRAYA_8; color_space CmsColorSpace::Gray, extra 1, channels 1, bps 1);
    PixelTypeDef!(pub const GRAYA_8_PREMUL; color_space CmsColorSpace::Gray, extra 1, channels 1, bps 1, premul);
    PixelTypeDef!(pub const GRAYA_16; color_space CmsColorSpace::Gray, extra 1, channels 1, bps 2);
    PixelTypeDef!(pub const GRAYA_16_PREMUL; color_space CmsColorSpace::Gray, extra 1, channels 1, bps 2, premul);
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
    PixelTypeDef!(pub const RGBA_8_PREMUL; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, premul);
    PixelTypeDef!(pub const RGBA_8_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, planar);
    PixelTypeDef!(pub const RGBA_16; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2);
    PixelTypeDef!(pub const RGBA_16_PREMUL; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, premul);
    PixelTypeDef!(pub const RGBA_16_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, planar);
    PixelTypeDef!(pub const RGBA_16_SE; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, endian16);

    PixelTypeDef!(pub const ARGB_8; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, swap_first);
    PixelTypeDef!(pub const ARGB_8_PREMUL; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, swap_first, premul);
    PixelTypeDef!(pub const ARGB_8_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, swap_first, planar);
    PixelTypeDef!(pub const ARGB_16; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, swap_first);
    PixelTypeDef!(pub const ARGB_16_PREMUL; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, swap_first, premul);

    PixelTypeDef!(pub const ABGR_8; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap);
    PixelTypeDef!(pub const ABGR_8_PREMUL; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, premul);
    PixelTypeDef!(pub const ABGR_8_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, planar);
    PixelTypeDef!(pub const ABGR_16; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap);
    PixelTypeDef!(pub const ABGR_16_PREMUL; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, premul);
    PixelTypeDef!(pub const ABGR_16_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, planar);
    PixelTypeDef!(pub const ABGR_16_SE; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, endian16);

    PixelTypeDef!(pub const BGRA_8; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, swap_first);
    PixelTypeDef!(pub const BGRA_8_PREMUL; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, swap_first, premul);
    PixelTypeDef!(pub const BGRA_8_PLANAR; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 1, do_swap, swap_first, planar);
    PixelTypeDef!(pub const BGRA_16; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, swap_first);
    PixelTypeDef!(pub const BGRA_16_PREMUL; color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 2, do_swap, swap_first, premul);
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
    PixelTypeDef!(pub const GRAYA_FLT; float, color_space CmsColorSpace::Gray, channels 1, bps 4, extra 1);
    PixelTypeDef!(pub const GRAYA_FLT_PREMUL; float, color_space CmsColorSpace::Gray, channels 1, bps 4, extra 1, premul);
    PixelTypeDef!(pub const RGB_FLT; float, color_space CmsColorSpace::Rgb, channels 3, bps 4);

    PixelTypeDef!(pub const RGBA_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4);
    PixelTypeDef!(pub const RGBA_FLT_PREMUL; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, premul);
    PixelTypeDef!(pub const ARGB_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, swap_first);
    PixelTypeDef!(pub const ARGB_FLT_PREMUL; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, swap_first, premul);
    PixelTypeDef!(pub const BGR_FLT; float, color_space CmsColorSpace::Rgb, channels 3, bps 4, do_swap);
    PixelTypeDef!(pub const BGRA_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap, swap_first);
    PixelTypeDef!(pub const BGRA_FLT_PREMUL; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap, swap_first, premul);
    PixelTypeDef!(pub const ABGR_FLT; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap);
    PixelTypeDef!(pub const ABGR_FLT_PREMUL; float, color_space CmsColorSpace::Rgb, extra 1, channels 3, bps 4, do_swap, premul);

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
