use super::CmsSignature as CmsSignature;

impl CmsSignature {
    pub const fn new(value: &[u8; 4]) -> CmsSignature {
        CmsSignature(u32::from_be_bytes(*value))
    }
}
impl From<CmsSignature> for u32 {
    fn from(item: CmsSignature) -> u32 {
        item.0
    }
}
impl From<u32> for CmsSignature {
    fn from(item: u32) -> Self {
        CmsSignature(item)
    }
}
impl From<&[u8; 4]> for CmsSignature {
    fn from(value: &[u8; 4]) -> Self {
        CmsSignature(u32::from_be_bytes(*value))
    }
}
impl From<&[u8; 3]> for CmsSignature {
    fn from(value: &[u8; 3]) -> Self {
        let mut result: [u8; 4] = [0x20; 4];
        result[..3].copy_from_slice(&*value);
        CmsSignature(u32::from_be_bytes(result))
    }
}
impl From<&[u8; 2]> for CmsSignature {
    fn from(value: &[u8; 2]) -> Self {
        let mut result: [u8; 4] = [0x20; 4];
        result[..2].copy_from_slice(&*value);
        CmsSignature(u32::from_be_bytes(result))
    }
}
impl From<&[u8; 1]> for CmsSignature {
    fn from(value: &[u8; 1]) -> Self {
        let mut result: [u8; 4] = [0x20; 4];
        result[..1].copy_from_slice(&*value);
        CmsSignature(u32::from_be_bytes(result))
    }
}
impl From<&[u8]> for CmsSignature {
    fn from(value: &[u8]) -> Self {
        let len = value.len();
        let mut result: [u8; 4] = [0x20; 4];
        if len >= 4 {
            result.copy_from_slice(&value[..4]);
            CmsSignature::from(&result)
        } else if len == 3 {
            result[..3].copy_from_slice(&value[..3]);
            CmsSignature::from(&result)
        } else if len == 2 {
            result[..2].copy_from_slice(&value[..2]);
            CmsSignature::from(&result)
        } else if len == 1 {
            result[..1].copy_from_slice(&value[..1]);
            CmsSignature::from(&result)
        } else {
            panic!()
        }
    }
}
impl From<&str> for CmsSignature {
    fn from(s: &str) -> Self {
        CmsSignature::from(s.as_bytes())
    }
}
