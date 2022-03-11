use std::mem::size_of;
use crate::plugin::*;
use crate::signatures as s;
use crate::*;
use paste::paste;
use std::convert::TryInto;
use std::io::*;
use std::io::{Read, Result, Write};

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

fn colorant_order_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
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
        items[(i*size_of::<u32>())..][..8].copy_from_slice(&read_s15f16_as_u8(reader)?);
    }

    Ok(n)
}

fn s15_f16_write(writer: &mut dyn Write, items: &[u8], count: usize) -> Result<()> {
    for i in 0..count {
        write_s15f16_from_u8(writer, items[(i*size_of::<u32>())..][..8].try_into().unwrap())?;
    }

    Ok(())
}

fn u16_f16_read(reader: &mut dyn Read, items: &mut [u8], length_in_bytes: usize) -> Result<usize> {
    let n = length_in_bytes / size_of::<u32>();

    for i in 0..n {
        let value = (read_u32(reader)?) as f64 / 65536.0;
        items[(i*size_of::<u32>())..][..8].copy_from_slice(&value.to_be_bytes());
    }

    Ok(n)
}

fn u16_f16_write(writer: &mut dyn Write, items: &[u8], count: usize) -> Result<()> {
    for i in 0..count {
        // Get the 8 bytes for the f64 value.
        let value = &items[(i*size_of::<f64>())..][..size_of::<f64>()];

        // Convert [u8; 8] form into f64
        let value = f64::from_be_bytes(value.try_into().unwrap());

        // Convert to U16F16
        let value = (value * 65536.0 + 0.5).floor() as U16F16;

        write_u32(writer, value)?;
    }

    Ok(())
}

fn signature_read(reader: &mut dyn Read, items: &mut [u8], _only_reads_one: usize) -> Result<usize> {
    items.copy_from_slice(&read_u32_as_u8(reader)?);

    Ok(1)
}

fn signature_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    write_u32_from_u8(writer, items.try_into().unwrap())
}

fn text_read(reader: &mut dyn Read, items: &mut [u8], tag_size: usize) -> Result<usize> {
    if items.len() <= tag_size { return Err(Error::from(ErrorKind::InvalidData)); }

    reader.read(&mut items[..tag_size])?;
    items[tag_size] = 0; // zero-terminated strings

    // verify we are still in ASCII land
    for i in items[..tag_size].iter() {
        if *i > 127 { return Err(Error::from(ErrorKind::InvalidData)); }
    }

    Ok(1)
}

fn text_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {

    // verify we are still in ASCII land
    for i in items.iter() {
        if *i > 127 { return Err(Error::from(ErrorKind::InvalidData)); }
    }

    writer.write(items)?;

    // and that the last byte is 0
    if items[items.len() - 1] != 0 { writer.write(&[0u8])?; }
    
    Ok(())
}

fn decide_xyz_type(_version: f64) -> Signature {
    signatures::tag_type::XYZ
}

fn decide_text_type(version: f64) -> Signature {
    match version {
        _v if _v >= 4.0 => s::tag_type::MULTI_LOCALIZED_UNICODE,
        _ => s::tag_type::TEXT,
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
    type_handler!(s::tag_type::SIGNATURE, Signature),
    type_handler!(s::tag_type::XYZ, Xyz),
    type_handler!(CORBIS_BROKEN_XYZ_TYPE, Xyz),
];

/* ------------------------------------------- Tag support main routines -------------------------------------------- */

pub struct TagListItem(Signature, TagDescriptor);

macro_rules! TagListItem {
    ($signature: expr, $element_count:expr, $supported_types:expr) => {
        TagListItem {
            0: $signature,
            1: TagDescriptor {
                element_count: $element_count,
                supported_types: &$supported_types,
                decide_type: None,
            },
        }
    };
    ($signature: expr, $element_count:expr, $supported_types:expr, $decide_type:expr) => {
        TagListItem {
            0: $signature,
            1: TagDescriptor {
                element_count: $element_count,
                supported_types: &$supported_types,
                decide_type: Some($decide_type),
            },
        }
    };
}

pub static SUPPORTED_TAGS: &[TagListItem] = &[
    TagListItem!(
        s::tag::RED_COLORANT,
        1,
        [s::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE],
        decide_xyz_type
    ),
    TagListItem!(
        s::tag::GREEN_COLORANT,
        1,
        [s::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE],
        decide_xyz_type
    ),
    TagListItem!(
        s::tag::BLUE_COLORANT,
        1,
        [s::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE],
        decide_xyz_type
    ),
    TagListItem!(s::tag::CHROMATIC_ADAPTATION, 9, [s::tag_type::S15_FIXED16_ARRAY]),
    TagListItem!(s::tag::CHROMATICITY, 1, [s::tag_type::CHROMATICITY]),
    TagListItem!(s::tag::COLORANT_ORDER, 1, [s::tag_type::COLORANT_ORDER]),
    TagListItem!(s::tag::TECHNOLOGY, 1, [s::tag_type::SIGNATURE]),
    TagListItem!(s::tag::COLORIMETRIC_INTENT_IMAGE_STATE, 1, [s::tag_type::SIGNATURE]),
    TagListItem!(s::tag::PERCEPTUAL_RENDERING_INTENT_GAMUT, 1, [s::tag_type::SIGNATURE]),
    TagListItem!(s::tag::SATURATION_RENDERING_INTENT_GAMUT, 1, [s::tag_type::SIGNATURE]),
    TagListItem!(s::tag::ARGYLL_ARTS, 9, [s::tag_type::S15_FIXED16_ARRAY]),
];
