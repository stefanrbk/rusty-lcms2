use std::io::Read;
use std::io::Write;
use std::io::Result;
use std::mem::size_of;

use super::*;

impl From<u32> for CmsTagBase {
    fn from(value: u32) -> Self {
        CmsTagBase {
            signature: CmsSignature::from(value),
            reserved: [0u8; 4],
        }
    }
}
impl From<CmsTagBase> for u32 {
    fn from(value: CmsTagBase) -> u32 {
        u32::from(value.signature)
    }
}
impl CmsTagBase {
    pub fn read(reader: &mut dyn Read) -> Result<CmsTagBase> {
        let mut buf = [0u8; size_of::<CmsSignature>()];
        let len = reader.read(&mut buf)?;
        if len < size_of::<CmsSignature>() {
            return Err(eof_error());
        }
        let sig = CmsSignature(u32::from_be_bytes(buf));

        let len = reader.read(&mut buf)?;
        if len < size_of::<CmsSignature>() {
            return Err(eof_error());
        }

        Ok(CmsTagBase {
            signature: sig,
            reserved: buf,
        })
    }
    pub fn write(self, writer: &mut dyn Write) -> Result<()> {
        write_u32(writer, u32::from(self.signature))?;
        write_u32(writer, u32::from_be_bytes(self.reserved))?;

        Ok(())
    }
}
