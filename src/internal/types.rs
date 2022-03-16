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
    let value = read_xyz_as_u8(reader)?;

    Ok((1, Box::new(value)))
}

fn xyz_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    write_xyz_from_u8(writer, items.try_into().unwrap())
}

fn chromaticity_read(reader: &mut dyn Read, _only_reads_one: usize) -> Result<(usize, Box<[u8]>)> {
    let num_channels = read_u16(reader)?;
    if num_channels != 3 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    let one = 1.0f64.to_be_bytes();

    _ = read_u16(reader)?;

    let mut result = [0u8; 72];

    result[CIExyYTripple::red][CIExyY::x].copy_from_slice(&read_s15f16_as_u8(reader)?);
    result[CIExyYTripple::red][CIExyY::y].copy_from_slice(&read_s15f16_as_u8(reader)?);
    result[CIExyYTripple::red][CIExyY::Y].copy_from_slice(&one);

    result[CIExyYTripple::green][CIExyY::x].copy_from_slice(&read_s15f16_as_u8(reader)?);
    result[CIExyYTripple::green][CIExyY::y].copy_from_slice(&read_s15f16_as_u8(reader)?);
    result[CIExyYTripple::green][CIExyY::Y].copy_from_slice(&one);

    result[CIExyYTripple::blue][CIExyY::x].copy_from_slice(&read_s15f16_as_u8(reader)?);
    result[CIExyYTripple::blue][CIExyY::y].copy_from_slice(&read_s15f16_as_u8(reader)?);
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
    let n = length_in_bytes / size_of::<u32>();
    let mut value = vec![0u8; n];

    for i in 0..n {
        value[(i * size_of::<u32>())..][..8].copy_from_slice(&read_s15f16_as_u8(reader)?);
    }

    Ok((n, value.into_boxed_slice()))
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

fn u16_f16_read(reader: &mut dyn Read, length_in_bytes: usize) -> Result<(usize, Box<[u8]>)> {
    let n = length_in_bytes / size_of::<u32>();
    let mut result = vec![0u8; n];

    for i in 0..n {
        let value = (read_u32(reader)?) as f64 / 65536.0;
        result[(i * size_of::<u32>())..][..8].copy_from_slice(&value.to_be_bytes());
    }

    Ok((n, result.into_boxed_slice()))
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

fn signature_read(reader: &mut dyn Read, _only_reads_one: usize) -> Result<(usize, Box<[u8]>)> {
    let result = read_u32_as_u8(reader)?;

    Ok((1, Box::new(result)))
}

fn signature_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    write_u32_from_u8(writer, items.try_into().unwrap())
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
    if tag_size < size_of::<u32>() {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    // The data is prefaced with a u32 flag value.
    let len_of_data = tag_size - size_of::<u32>();

    // The resulting data will be prefaced with the length and flag value (flags included in tag_size already!)
    let mut result = vec![0u8; tag_size + size_of::<u32>()];

    result[ICCData::length].copy_from_slice(&len_of_data.to_be_bytes());
    result[ICCData::flag].copy_from_slice(&read_u32_as_u8(reader)?);

    match reader.read(&mut result[ICCData::data])? {
        len if len == size_of::<u8>() * len_of_data => Ok((1, result.into_boxed_slice())),
        _ => Err(Error::from(ErrorKind::UnexpectedEof)),
    }
}

fn data_write(writer: &mut dyn Write, items: &[u8], _only_writes_one: usize) -> Result<()> {
    
    Ok(())
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
