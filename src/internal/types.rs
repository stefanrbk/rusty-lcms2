use crate::plugin::*;
use crate::*;
use std::io::{Read, Result, Write};

// Some broken types
const CORBIS_BROKEN_XYZ_TYPE: Signature = Signature::new(&[0x17, 0xA5, 0x05, 0xB8]);
const MONACO_BROKEN_CURVE_TYPE: Signature = Signature::new(&[0x94, 0x78, 0xEE, 0x00]);

pub struct XyzHandler {}

impl TagTypeHandler for XyzHandler {
    type TagType = CIEXYZ;
    fn get_signature() -> Signature {
        todo!()
    }
    fn get_version() -> u32 {
        todo!()
    }
    fn read(
        reader: &mut dyn Read,
        items: &mut [Self::TagType],
        _only_reads_one: usize,
    ) -> Result<usize> {
        items[0] = read_xyz(reader)?;
        Ok(1)
    }
    fn write(writer: &mut dyn Write, items: &[Self::TagType], count: usize) -> Result<()> {
        write_xyz(writer, items[0])
    }
}
fn decide_xyz_type(_version: f64) -> Signature {
    signatures::tag_type::XYZ
}

pub const XYZ_HANDLER: &dyn TagTypeHandler<TagType = CIEXYZ> = &XyzHandler {};

/* ------------------------------------------- Tag support main routines -------------------------------------------- */

pub struct TagListItem(Signature, TagDescriptor);

pub static SUPPORTED_TAGS: &[TagListItem] = &[
    TagListItem {
        0: signatures::tag::RED_COLORANT,
        1: TagDescriptor {
            element_count: 1,
            supported_types: &[signatures::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE],
            decide_type: Some(decide_xyz_type),
        },
    },
    TagListItem {
        0: signatures::tag::GREEN_COLORANT,
        1: TagDescriptor {
            element_count: 1,
            supported_types: &[signatures::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE],
            decide_type: Some(decide_xyz_type),
        },
    },
    TagListItem {
        0: signatures::tag::BLUE_COLORANT,
        1: TagDescriptor {
            element_count: 1,
            supported_types: &[signatures::tag_type::XYZ, CORBIS_BROKEN_XYZ_TYPE],
            decide_type: Some(decide_xyz_type),
        },
    },
];
