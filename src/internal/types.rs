use crate::plugin::*;
use crate::signatures as s;
use crate::*;
use paste::paste;
use std::convert::TryInto;
use std::io::*;
use std::io::{Read, Result, Write};
use std::mem::size_of;

// Some broken types
const CORBIS_BROKEN_XYZ_TYPE: Signature = Signature::new(&[0x17, 0xA5, 0x05, 0xB8]);
const MONACO_BROKEN_CURVE_TYPE: Signature = Signature::new(&[0x94, 0x78, 0xEE, 0x00]);

fn xyz_read(reader: &mut dyn Read, items: &mut [u8], _only_reads_one: usize) -> Result<usize> {
    items.copy_from_slice(&read_xyz_as_u8(reader)?);

    Ok(1)
}

fn xyz_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    write_xyz_from_u8(writer, items.try_into().unwrap())
}

fn chromaticity_read(
    reader: &mut dyn Read,
    items: &mut [u8],
    _only_reads_one: usize,
) -> Result<usize> {
    let num_channels = read_u16(reader)?;
    if num_channels != 3 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let one = 1.0f64.to_be_bytes();

    _ = read_u16(reader)?;

    items[CIExyYTripple::red][CIExyY::x].copy_from_slice(&read_s15f16_as_u8(reader)?);
    items[CIExyYTripple::red][CIExyY::y].copy_from_slice(&read_s15f16_as_u8(reader)?);
    items[CIExyYTripple::red][CIExyY::Y].copy_from_slice(&one);

    items[CIExyYTripple::green][CIExyY::x].copy_from_slice(&read_s15f16_as_u8(reader)?);
    items[CIExyYTripple::green][CIExyY::y].copy_from_slice(&read_s15f16_as_u8(reader)?);
    items[CIExyYTripple::green][CIExyY::Y].copy_from_slice(&one);

    items[CIExyYTripple::blue][CIExyY::x].copy_from_slice(&read_s15f16_as_u8(reader)?);
    items[CIExyYTripple::blue][CIExyY::y].copy_from_slice(&read_s15f16_as_u8(reader)?);
    items[CIExyYTripple::blue][CIExyY::Y].copy_from_slice(&one);

    Ok(1)
}

fn chromaticity_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    write_u16(writer, 3)?;
    write_u16(writer, 0)?;

    save_one_chromaticity(&items[CIExyYTripple::red], writer)?;
    save_one_chromaticity(&items[CIExyYTripple::red], writer)?;
    save_one_chromaticity(&items[CIExyYTripple::red], writer)?;

    Ok(())
}

fn colorant_order_read(
    reader: &mut dyn Read,
    items: &mut [u8],
    _only_reads_one: usize,
) -> Result<usize> {
    let count = read_u32(reader)? as usize;
    if count > MAX_CHANNELS as usize || items.len() < count {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    // Set all values to 0xFF as that is the end of the data once writen to.
    items.iter_mut().for_each(|x| *x = 0xFF);

    match reader.read(&mut items[0..count])? {
        len if len == size_of::<u8>() * count => Ok(1),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

fn colorant_order_write(
    writer: &mut dyn Write,
    items: &[u8],
    _only_writes_one: usize,
) -> Result<()> {
    let mut count = 0;
    for i in 0..items.len() {
        if items[i] != 0xFF {
            count += 1;
        } else {
            break;
        }
    }
    if count > MAX_CHANNELS {
        count = MAX_CHANNELS;
    }

    let items = &items[0..count as usize];

    write_u32(writer, count)?;
    match writer.write(items)? {
        len if len == size_of::<u8>() * count as usize => Ok(()),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

fn s15_f16_read(reader: &mut dyn Read, items: &mut [u8], length_in_bytes: usize) -> Result<usize> {
    let n = length_in_bytes / size_of::<u32>();

    for i in 0..n {
        items[(i * size_of::<u32>())..][..8].copy_from_slice(&read_s15f16_as_u8(reader)?);
    }

    Ok(n)
}

fn s15_f16_write(writer: &mut dyn Write, items: &[u8], count: usize) -> Result<()> {
    for i in 0..count {
        write_s15f16_from_u8(
            writer,
            items[(i * size_of::<u32>())..][..8].try_into().unwrap(),
        )?;
    }

    Ok(())
}

fn u16_f16_read(reader: &mut dyn Read, items: &mut [u8], length_in_bytes: usize) -> Result<usize> {
    let n = length_in_bytes / size_of::<u32>();

    for i in 0..n {
        let value = (read_u32(reader)?) as f64 / 65536.0;
        items[(i * size_of::<u32>())..][..8].copy_from_slice(&value.to_be_bytes());
    }

    Ok(n)
}

fn u16_f16_write(writer: &mut dyn Write, items: &[u8], count: usize) -> Result<()> {
    for i in 0..count {
        // Get the 8 bytes for the f64 value.
        let value = &items[(i * size_of::<f64>())..][..size_of::<f64>()];

        // Convert [u8; 8] form into f64
        let value = f64::from_be_bytes(value.try_into().unwrap());

        // Convert to U16F16
        let value = (value * 65536.0 + 0.5).floor() as U16F16;

        write_u32(writer, value)?;
    }

    Ok(())
}

fn signature_read(
    reader: &mut dyn Read,
    items: &mut [u8],
    _only_reads_one: usize,
) -> Result<usize> {
    items.copy_from_slice(&read_u32_as_u8(reader)?);

    Ok(1)
}

fn signature_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    write_u32_from_u8(writer, items.try_into().unwrap())
}

fn text_read(reader: &mut dyn Read, items: &mut [u8], tag_size: usize) -> Result<usize> {
    if items.len() <= tag_size {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    reader.read(&mut items[..tag_size])?;
    items[tag_size] = 0; // zero-terminated strings

    // verify we are still in ASCII land
    for i in items[..tag_size].iter() {
        if *i > 127 {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }

    Ok(1)
}

fn text_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    // verify we are still in ASCII land
    for i in items.iter() {
        if *i > 127 {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }

    writer.write(items)?;

    // and that the last byte is 0
    if items[items.len() - 1] != 0 {
        writer.write(&[0u8])?;
    }
    Ok(())
}

fn decide_xyz_type(_version: f64, _data: &[i8]) -> Signature {
    signatures::tag_type::XYZ
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

fn save_one_chromaticity(item: &[u8], writer: &mut dyn Write) -> Result<()> {
    write_s15f16_from_u8(writer, item[CIExyY::x].try_into().unwrap())?;
    write_s15f16_from_u8(writer, item[CIExyY::y].try_into().unwrap())?;

    Ok(())
}

macro_rules! type_handler {
    ($sig: expr, $name: ident) => {
        paste! {
            TagTypeHandler {
                signature: $sig,
                version: 0,
                read: [<$name:snake:lower _read>],
                write: [<$name:snake:lower _write>],
            }
        }
    };
    ($sig: expr, $name: ident, $version: expr) => {
        paste! {
            TagTypeHandler {
                signature: $sig,
                version: $version,
                read: [<$name:lower _read>],
                write: [<$name:lower _write>],
            }
        }
    };
}

pub static SUPPORTED_TAG_TYPES: &[TagTypeHandler] = &[
    type_handler!(s::tag_type::CHROMATICITY, Chromaticity),
    type_handler!(s::tag_type::COLORANT_ORDER, ColorantOrder),
    type_handler!(s::tag_type::S15_FIXED16_ARRAY, S15F16),
    type_handler!(s::tag_type::U16_FIXED16_ARRAY, U16F16),
    type_handler!(s::tag_type::TEXT, Text),
    type_handler!(s::tag_type::SIGNATURE, Signature),
    type_handler!(s::tag_type::XYZ, Xyz),
    type_handler!(CORBIS_BROKEN_XYZ_TYPE, Xyz),
];

/* ------------------------------------------- Tag support main routines -------------------------------------------- */

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

// pub fn register_tag_plugin(plugin: Option<PluginTag>) -> Result<()> {
//     match plugin {
//         None => 
//     }
// }
