use crate::Signature;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::mem::size_of;

use super::{eof_error, write_u32};

pub struct TagBase {
    pub signature: Signature,
    pub reserved: [u8; 4],
}

impl From<u32> for TagBase {
    fn from(value: u32) -> Self {
        Self {
            signature: Signature::from(value),
            reserved: [0u8; 4],
        }
    }
}
impl From<TagBase> for u32 {
    fn from(value: TagBase) -> Self {
        Self::from(value.signature)
    }
}
impl TagBase {
    pub fn read(reader: &mut dyn Read) -> Result<Self> {
        let mut buf = [0u8; size_of::<Signature>()];
        let len = reader.read(&mut buf)?;
        if len < size_of::<Signature>() {
            return Err(eof_error());
        }
        let sig = Signature::new(&buf);

        let len = reader.read(&mut buf)?;
        if len < size_of::<Signature>() {
            return Err(eof_error());
        }

        Ok(Self {
            signature: sig,
            reserved: buf,
        })
    }
    pub fn read_type_base(reader: &mut dyn Read) -> Signature {
        let value = Self::read(reader);
        if value.is_err() {
            return Default::default();
        }
        return value.unwrap().signature;
    }
    pub fn write(self, writer: &mut dyn Write) -> Result<()> {
        write_u32(writer, u32::from(self.signature))?;
        write_u32(writer, u32::from_ne_bytes(self.reserved))?;

        Ok(())
    }
}
