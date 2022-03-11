use crate::internal::*;
use crate::Signature;
use crate::plugin::*;
use crate::signatures as s;

fn decide_xyz_type(_version: f64, _data: &[i8]) -> Signature {
    s::tag_type::XYZ
}

fn decide_text_type(version: f64, _data: &[i8]) -> Signature {
    match version {
        _v if _v >= 4.0 => s::tag_type::MULTI_LOCALIZED_UNICODE,
        _ => s::tag_type::TEXT,
    }
}

fn decide_lut_type_a_to_b(version: f64, data: &[i8]) -> Signature {
    if data.len() < 1 {
        panic!("Expected at least 1 i8 item in \"data\".");
    }
    let save_as_8_bits = data[0];

    match version {
        _v if _v < 4.0 => match save_as_8_bits {
            _b if _b != 0 => s::tag_type::LUT8,
            _ => s::tag_type::LUT16,
        },
        _ => s::tag_type::LUTA_TO_B,
    }
}

fn decide_lut_type_b_to_a(version: f64, data: &[i8]) -> Signature {
    if data.len() < 1 {
        panic!("Expected at least 1 i8 item in \"data\".");
    }
    let save_as_8_bits = data[0];

    match version {
        _v if _v < 4.0 => match save_as_8_bits {
            _b if _b != 0 => s::tag_type::LUT8,
            _ => s::tag_type::LUT16,
        },
        _ => s::tag_type::LUTB_TO_A,
    }
}

fn decide_curve_type(version: f64, data: &[i8]) -> Signature {
    if data.len() < 2 {
        panic!("Expected at least 2 i8 items in \"data\".");
    }
    let (segments, segment_type) = (data[0], data[1]);

    match version {
        _v if _v < 4.0 => s::tag_type::CURVE,
        _ => match segments {
            _s if _s != 1 => s::tag_type::CURVE,
            _ => match segment_type {
                _t if _t < 0 => s::tag_type::CURVE,
                _t if _t > 5 => s::tag_type::CURVE,
                _ => s::tag_type::PARAMETRIC_CURVE,
            },
        },
    }
}

fn decide_text_desc_type(version: f64, _data: &[i8]) -> Signature {
    match version {
        _v if _v >= 4.0 => s::tag_type::MULTI_LOCALIZED_UNICODE,
        _ => s::tag_type::TEXT_DESCRIPTION,
    }
}

pub static SUPPORTED_TAGS: &[TagListItem] = &[
    TagListItem!(
        s::tag::A_TO_B0,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTA_TO_B,
            s::tag_type::LUT8
        ],
        decide_lut_type_a_to_b
    ), // s::tag::A_TO_B0
    TagListItem!(
        s::tag::A_TO_B1,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTA_TO_B,
            s::tag_type::LUT8
        ],
        decide_lut_type_a_to_b
    ), // s::tag::A_TO_B1
    TagListItem!(
        s::tag::A_TO_B2,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTA_TO_B,
            s::tag_type::LUT8
        ],
        decide_lut_type_a_to_b
    ), // s::tag::A_TO_B2
    TagListItem!(
        s::tag::B_TO_A0,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTB_TO_A,
            s::tag_type::LUT8
        ],
        decide_lut_type_b_to_a
    ), // s::tag::B_TO_A0
    TagListItem!(
        s::tag::B_TO_A1,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTB_TO_A,
            s::tag_type::LUT8
        ],
        decide_lut_type_b_to_a
    ), // s::tag::B_TO_A1
    TagListItem!(
        s::tag::B_TO_A2,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTB_TO_A,
            s::tag_type::LUT8
        ],
        decide_lut_type_b_to_a
    ), // s::tag::B_TO_A2
    //
    //
    TagListItem!(
        s::tag::RED_COLORANT,
        1,
        [s::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE],
        decide_xyz_type
    ), // s::tag::RED_COLORANT
    TagListItem!(
        s::tag::GREEN_COLORANT,
        1,
        [s::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE],
        decide_xyz_type
    ), // s::tag::GREEN_COLORANT
    TagListItem!(
        s::tag::BLUE_COLORANT,
        1,
        [s::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE],
        decide_xyz_type
    ), // s::tag::BLUE_COLORANT
    //
    //
    TagListItem!(
        s::tag::RED_TRC,
        1,
        [
            s::tag_type::CURVE,
            s::tag_type::PARAMETRIC_CURVE,
            MONACO_BROKEN_CURVE_TYPE
        ],
        decide_curve_type
    ), // s::tag::RED_TRC
    TagListItem!(
        s::tag::GREEN_TRC,
        1,
        [
            s::tag_type::CURVE,
            s::tag_type::PARAMETRIC_CURVE,
            MONACO_BROKEN_CURVE_TYPE
        ],
        decide_curve_type
    ), // s::tag::GREEN_TRC
    TagListItem!(
        s::tag::BLUE_TRC,
        1,
        [
            s::tag_type::CURVE,
            s::tag_type::PARAMETRIC_CURVE,
            MONACO_BROKEN_CURVE_TYPE
        ],
        decide_curve_type
    ), // s::tag::BLUE_TRC
    //
    //
    TagListItem!(s::tag::CALIBRATION_DATE_TIME, 1, [s::tag_type::DATE_TIME]),
    TagListItem!(s::tag::CHAR_TARGET, 1, [s::tag_type::TEXT]),
    //
    //
    TagListItem!(
        s::tag::CHROMATIC_ADAPTATION,
        9,
        [s::tag_type::S15_FIXED16_ARRAY]
    ), // s::tag::CHROMATIC_ADAPTATION
    TagListItem!(s::tag::CHROMATICITY, 1, [s::tag_type::CHROMATICITY]),
    TagListItem!(s::tag::COLORANT_ORDER, 1, [s::tag_type::COLORANT_ORDER]),
    TagListItem!(s::tag::COLORANT_TABLE, 1, [s::tag_type::COLORANT_TABLE]),
    TagListItem!(s::tag::COLORANT_TABLE_OUT, 1, [s::tag_type::COLORANT_TABLE]),
    //
    //
    TagListItem!(
        s::tag::COPYRIGHT,
        1,
        [
            s::tag_type::TEXT,
            s::tag_type::MULTI_LOCALIZED_UNICODE,
            s::tag_type::TEXT_DESCRIPTION
        ],
        decide_text_type
    ), // s::tag::COPYRIGHT
    TagListItem!(s::tag::DATE_TIME, 1, [s::tag_type::DATE_TIME]),
    //
    //
    TagListItem!(
        s::tag::DEVICE_MFG_DESC,
        1,
        [
            s::tag_type::TEXT_DESCRIPTION,
            s::tag_type::MULTI_LOCALIZED_UNICODE,
            s::tag_type::TEXT,
        ],
        decide_text_desc_type
    ), // s::tag::DEVICE_MFG_DESC
    TagListItem!(
        s::tag::DEVICE_MODEL_DESC,
        1,
        [
            s::tag_type::TEXT_DESCRIPTION,
            s::tag_type::MULTI_LOCALIZED_UNICODE,
            s::tag_type::TEXT,
        ],
        decide_text_desc_type
    ), // s::tag::DEVICE_MODEL_DESC
    //
    //
    TagListItem!(
        s::tag::GAMUT,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTB_TO_A,
            s::tag_type::LUT8
        ],
        decide_lut_type_b_to_a
    ), // s::tag::GAMUT
    //
    //
    TagListItem!(
        s::tag::GRAY_TRC,
        1,
        [s::tag_type::CURVE, s::tag_type::PARAMETRIC_CURVE,],
        decide_curve_type
    ), // s::tag::GRAY_TRC
    TagListItem!(s::tag::LUMINANCE, 1, [s::tag_type::XYZ]),
    //
    //
    TagListItem!(
        s::tag::MEDIA_BLACK_POINT,
        1,
        [s::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE]
    ), // s::tag::MEDIA_BLACK_POINT
    TagListItem!(
        s::tag::MEDIA_WHITE_POINT,
        1,
        [s::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE]
    ), // s::tag::MEDIA_WHITE_POINT
    //
    //
    TagListItem!(s::tag::NAMED_COLOR2, 1, [s::tag_type::NAMED_COLOR2]),
    //
    //
    TagListItem!(
        s::tag::PREVIEW0,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTB_TO_A,
            s::tag_type::LUT8
        ],
        decide_lut_type_b_to_a
    ), // s::tag::PREVIEW0
    TagListItem!(
        s::tag::PREVIEW1,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTB_TO_A,
            s::tag_type::LUT8
        ],
        decide_lut_type_b_to_a
    ), // s::tag::PREVIEW1
    TagListItem!(
        s::tag::PREVIEW2,
        1,
        [
            s::tag_type::LUT16,
            s::tag_type::LUTB_TO_A,
            s::tag_type::LUT8
        ],
        decide_lut_type_b_to_a
    ), // s::tag::PREVIEW2
    //
    //
    TagListItem!(
        s::tag::PROFILE_DESCRIPTION,
        1,
        [
            s::tag_type::TEXT_DESCRIPTION,
            s::tag_type::MULTI_LOCALIZED_UNICODE,
            s::tag_type::TEXT,
        ],
        decide_text_desc_type
    ), // s::tag::PROFILE_DESCRIPTION
    TagListItem!(
        s::tag::PROFILE_SEQUENCE_DESC,
        1,
        [s::tag_type::PROFILE_SEQUENCE_DESC]
    ), // s::tag::PROFILE_SEQUENCE_DESC
    TagListItem!(s::tag::TECHNOLOGY, 1, [s::tag_type::SIGNATURE]),
    //
    //
    TagListItem!(
        s::tag::COLORIMETRIC_INTENT_IMAGE_STATE,
        1,
        [s::tag_type::SIGNATURE]
    ),// s::tag::COLORIMETRIC_INTENT_IMAGE_STATE
    TagListItem!(
        s::tag::PERCEPTUAL_RENDERING_INTENT_GAMUT,
        1,
        [s::tag_type::SIGNATURE]
    ),// s::tag::PERCEPTUAL_RENDERING_INTENT_GAMUT
    TagListItem!(
        s::tag::SATURATION_RENDERING_INTENT_GAMUT,
        1,
        [s::tag_type::SIGNATURE]
    ),// s::tag::SATURATION_RENDERING_INTENT_GAMUT
    //
    //
    TagListItem!(s::tag::MEASUREMENT, 1, [s::tag_type::MEASUREMENT]),
    //
    //
    TagListItem!(s::tag::PS2_CRD0, 1, [s::tag_type::DATA]),
    TagListItem!(s::tag::PS2_CRD1, 1, [s::tag_type::DATA]),
    TagListItem!(s::tag::PS2_CRD2, 1, [s::tag_type::DATA]),
    TagListItem!(s::tag::PS2_CRD3, 1, [s::tag_type::DATA]),
    TagListItem!(s::tag::PS2_CSA, 1, [s::tag_type::DATA]),
    TagListItem!(s::tag::PS2_RENDERING_INTENT, 1, [s::tag_type::DATA]),
    //
    //
    TagListItem!(
        s::tag::VIEWING_COND_DESC,
        1,
        [
            s::tag_type::TEXT_DESCRIPTION,
            s::tag_type::MULTI_LOCALIZED_UNICODE,
            s::tag_type::TEXT,
        ],
        decide_text_desc_type
    ), // s::tag::VIEWING_COND_DESC
    //
    //
    TagListItem!(s::tag::UCR_BG, 1, [s::tag_type::UCR_BG]),
    TagListItem!(s::tag::CRD_INFO, 1, [s::tag_type::CRD_INFO]),
    //
    //
    TagListItem!(s::tag::D_TO_B0, 1, [s::tag_type::MULTI_PROCESS_ELEMENT]),
    TagListItem!(s::tag::D_TO_B1, 1, [s::tag_type::MULTI_PROCESS_ELEMENT]),
    TagListItem!(s::tag::D_TO_B2, 1, [s::tag_type::MULTI_PROCESS_ELEMENT]),
    TagListItem!(s::tag::D_TO_B3, 1, [s::tag_type::MULTI_PROCESS_ELEMENT]),
    TagListItem!(s::tag::B_TO_D0, 1, [s::tag_type::MULTI_PROCESS_ELEMENT]),
    TagListItem!(s::tag::B_TO_D1, 1, [s::tag_type::MULTI_PROCESS_ELEMENT]),
    TagListItem!(s::tag::B_TO_D2, 1, [s::tag_type::MULTI_PROCESS_ELEMENT]),
    TagListItem!(s::tag::B_TO_D3, 1, [s::tag_type::MULTI_PROCESS_ELEMENT]),
    //
    //
    TagListItem!(s::tag::SCREENING_DESC, 1, [s::tag_type::TEXT_DESCRIPTION]),
    TagListItem!(s::tag::VIEWING_CONDITIONS, 1, [s::tag_type::VIEWING_CONDITIONS]),
    //
    //
    TagListItem!(s::tag::SCREENING, 1, [s::tag_type::SCREENING]),
    TagListItem!(s::tag::VCGT, 1, [s::tag_type::VCGT]),
    TagListItem!(s::tag::META, 1, [s::tag_type::DICT]),
    TagListItem!(s::tag::PROFILE_SEQUENCE_ID, 1, [s::tag_type::PROFILE_SEQUENCE_ID]),
    //
    //
    TagListItem!(s::tag::PROFILE_DESCRIPTION_ML, 9, [s::tag_type::MULTI_LOCALIZED_UNICODE]),
    TagListItem!(s::tag::ARGYLL_ARTS, 9, [s::tag_type::S15_FIXED16_ARRAY]),
];
