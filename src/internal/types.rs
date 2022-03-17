use crate::internal::*;
use crate::plugin::*;
use crate::signatures as s;
use crate::*;
use paste::paste;
use std::convert::TryInto;
use std::io::*;
use std::io::{Read, Result, Write};
use std::mem::size_of;

fn xyz_read(reader: &mut dyn Read, _only_reads_one: usize) -> Result<(usize, Box<[u8]>)> {
    let value = read_xyz(reader)?;

    let mut buf = [0u8; size_of::<CIEXYZ>()];
    buf[CIEXYZ::X].copy_from_slice(&value.X.to_ne_bytes());
    buf[CIEXYZ::Y].copy_from_slice(&value.Y.to_ne_bytes());
    buf[CIEXYZ::Z].copy_from_slice(&value.Z.to_ne_bytes());

    Ok((1, Box::new(buf)))
}

fn xyz_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    let value = CIEXYZ {
        X: f64::from_ne_bytes(items[CIEXYZ::X].try_into().unwrap()),
        Y: f64::from_ne_bytes(items[CIEXYZ::Y].try_into().unwrap()),
        Z: f64::from_ne_bytes(items[CIEXYZ::Z].try_into().unwrap()),
    };
    write_xyz(writer, value)
}

fn chromaticity_read(reader: &mut dyn Read, _only_reads_one: usize) -> Result<(usize, Box<[u8]>)> {
    let num_channels = read_u16(reader)?;
    if num_channels != 3 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let one = 1.0f64.to_ne_bytes();

    _ = read_u16(reader)?;

    let mut result = [0u8; 72];

    result[CIExyYTripple::red][CIExyY::x].copy_from_slice(&read_s15f16(reader)?.to_ne_bytes());
    result[CIExyYTripple::red][CIExyY::y].copy_from_slice(&read_s15f16(reader)?.to_ne_bytes());
    result[CIExyYTripple::red][CIExyY::Y].copy_from_slice(&one);

    result[CIExyYTripple::green][CIExyY::x].copy_from_slice(&read_s15f16(reader)?.to_ne_bytes());
    result[CIExyYTripple::green][CIExyY::y].copy_from_slice(&read_s15f16(reader)?.to_ne_bytes());
    result[CIExyYTripple::green][CIExyY::Y].copy_from_slice(&one);

    result[CIExyYTripple::blue][CIExyY::x].copy_from_slice(&read_s15f16(reader)?.to_ne_bytes());
    result[CIExyYTripple::blue][CIExyY::y].copy_from_slice(&read_s15f16(reader)?.to_ne_bytes());
    result[CIExyYTripple::blue][CIExyY::Y].copy_from_slice(&one);

    Ok((1, Box::new(result)))
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
    _only_reads_one: usize,
) -> Result<(usize, Box<[u8]>)> {
    let count = read_u32(reader)? as usize;

    // Set all values to 0xFF as that is the end of the data once writen to.
    let mut result = vec![0xFFu8; count];

    if count > MAX_CHANNELS as usize {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    match reader.read(&mut result)? {
        len if len == size_of::<u8>() * count => Ok((1, result.into_boxed_slice())),
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

fn s15_f16_read(reader: &mut dyn Read, length_in_bytes: usize) -> Result<(usize, Box<[u8]>)> {
    const SIZE_U32: usize = size_of::<u32>();
    let n = length_in_bytes / SIZE_U32;
    let mut value = vec![0u8; n];

    for i in 0..n {
        value[(i * SIZE_U32)..][..8].copy_from_slice(&read_s15f16(reader)?.to_ne_bytes());
    }

    Ok((n, value.into_boxed_slice()))
}

fn s15_f16_write(writer: &mut dyn Write, items: &[u8], count: usize) -> Result<()> {
    for i in 0..count {
        write_s15f16(
            writer,
            f64::from_ne_bytes(items[(i * size_of::<u32>())..][..8].try_into().unwrap()),
        )?;
    }

    Ok(())
}

fn u16_f16_read(reader: &mut dyn Read, length_in_bytes: usize) -> Result<(usize, Box<[u8]>)> {
    const SIZE_U32: usize = size_of::<u32>();
    let n = length_in_bytes / SIZE_U32;
    let mut result = vec![0u8; n];

    for i in 0..n {
        let value = (read_u32(reader)?) as f64 / 65536.0;
        result[(i * SIZE_U32)..][..8].copy_from_slice(&value.to_ne_bytes());
    }

    Ok((n, result.into_boxed_slice()))
}

fn u16_f16_write(writer: &mut dyn Write, items: &[u8], count: usize) -> Result<()> {
    for i in 0..count {
        // Get the 8 bytes for the f64 value.
        let value = &items[(i * size_of::<f64>())..][..size_of::<f64>()];

        // Convert [u8; 8] form into f64
        let value = f64::from_ne_bytes(value.try_into().unwrap());

        // Convert to U16F16
        let value = (value * 65536.0 + 0.5).floor() as U16F16;

        write_u32(writer, value)?;
    }

    Ok(())
}

fn signature_read(reader: &mut dyn Read, _only_reads_one: usize) -> Result<(usize, Box<[u8]>)> {
    let result = read_u32(reader)?;

    Ok((1, Box::new(result.to_ne_bytes())))
}

fn signature_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    let items = &items[..size_of::<u32>()];
    write_u32(writer, u32::from_ne_bytes(items.try_into().unwrap()))
}

fn text_read(reader: &mut dyn Read, tag_size: usize) -> Result<(usize, Box<[u8]>)> {
    let mut result = vec![0u8; tag_size + 1];

    reader.read(&mut result[..tag_size])?;
    result[tag_size] = 0; // zero-terminated strings

    // verify we are still in ASCII land
    for i in result[..tag_size].iter() {
        if *i > 127 {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }

    Ok((1, result.into_boxed_slice()))
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

fn data_read(reader: &mut dyn Read, tag_size: usize) -> Result<(usize, Box<[u8]>)> {
    const SIZE_U32: usize = size_of::<u32>();
    const SIZE_U8: usize = size_of::<u8>();

    if tag_size < SIZE_U32 {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    // The data is prefaced with a u32 flag value.
    let len_of_data = tag_size - SIZE_U32;

    // The resulting data will be prefaced with the length and flag value (flags included in tag_size already!)
    let mut result = vec![0u8; tag_size + SIZE_U32];

    result[ICCData::length].copy_from_slice(&len_of_data.to_ne_bytes());
    result[ICCData::flag].copy_from_slice(&read_u32(reader)?.to_ne_bytes());

    match reader.read(&mut result[ICCData::data])? {
        len if len == SIZE_U8 * len_of_data => Ok((1, result.into_boxed_slice())),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

fn data_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    let len = u32::from_ne_bytes(items[ICCData::length].try_into().unwrap());
    write_u32(
        writer,
        u32::from_ne_bytes(items[ICCData::flag].try_into().unwrap()),
    )?;

    if items[ICCData::data].len() != len as usize {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    writer.write(&items[ICCData::data])?;

    Ok(())
}

fn text_description_read(reader: &mut dyn Read, tag_size: usize) -> Result<(usize, Box<[u8]>)> {
    const SIZE_U8: usize = size_of::<u8>();
    const SIZE_U16: usize = size_of::<u16>();
    const SIZE_U32: usize = size_of::<u32>();

    let mut tag_size = tag_size;

    if tag_size < SIZE_U32 {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    // Read the length of ASCII data
    let ascii_count = read_u32(reader)? as usize;

    if tag_size < ascii_count {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    let mut text = vec![0u8; tag_size + 1];
    reader.read_exact(&mut text.as_mut_slice())?;
    tag_size -= ascii_count;

    // Make sure there is a terminator
    text[ascii_count] = 0;

    let result = Ok((1, text.into_boxed_slice()));

    if tag_size < 2 * SIZE_U32 {
        return result;
    }

    /* Code for when we actually use the Unicode */

    // let unicode_code = if let Ok(code) = read_u32(reader) {
    //     code
    // } else {
    //     return result;
    // };

    if let Err(_) = read_u32(reader) { return result; }

    let unicode_count = if let Ok(count) = read_u32(reader) {
        count as usize
    } else {
        return result;
    };

    tag_size -= 2 * SIZE_U32;

    if tag_size < unicode_count * SIZE_U16 {
        return result;
    }

    let mut dummy = vec![0u8; unicode_count * SIZE_U16];
    if let Err(_) = reader.read(&mut dummy.as_mut_slice()) { return result; }
    tag_size -= unicode_count * SIZE_U16;

    // Skip ScriptCode

    if tag_size >= SIZE_U16 + SIZE_U8 + 67 {
        if let Err(_) = read_u16(reader) { return result; }
        if let Err(_) = read_u8(reader) { return result; }

        let mut dummy = [0u8; 67];
        if let Err(_) = reader.read(&mut dummy) { return result; }
    }

    result
}

// fn text_description_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
//     let filler = [0u8; 68];

//     Ok(())
// }

fn save_one_chromaticity(item: &[u8], writer: &mut dyn Write) -> Result<()> {
    write_s15f16(
        writer,
        f64::from_ne_bytes(item[CIExyY::x].try_into().unwrap()),
    )?;
    write_s15f16(
        writer,
        f64::from_ne_bytes(item[CIExyY::y].try_into().unwrap()),
    )?;

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
    type_handler!(s::tag_type::DATA, Data),
    type_handler!(s::tag_type::XYZ, Xyz),
    type_handler!(CORBIS_BROKEN_XYZ_TYPE, Xyz),
];
