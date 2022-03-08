use std::convert::TryInto;
use crate::plugin::*;
use crate::signatures as s;
use crate::*;
use std::io::{Read, Result, Write};

// Some broken types
const CORBIS_BROKEN_XYZ_TYPE: Signature = Signature::new(&[0x17, 0xA5, 0x05, 0xB8]);
const MONACO_BROKEN_CURVE_TYPE: Signature = Signature::new(&[0x94, 0x78, 0xEE, 0x00]);

pub const XyzHandler: TagTypeHandler = TagTypeHandler {
    signature: todo!(),
    version: todo!(),
    read: xyz_read,
    write: xyz_write,
};

fn xyz_read(reader: &mut dyn Read, items: &mut [u8], _only_reads_one: usize) -> Result<usize> {
    items.copy_from_slice(&read_xyz(reader)?);

    Ok(1)
}

fn xyz_write(writer: &mut dyn Write, items: &[u8], count: usize) -> Result<()> {
    write_xyz(writer, items.try_into().unwrap())
}
fn decide_xyz_type(_version: f64) -> Signature {
    signatures::tag_type::XYZ
}

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
];
