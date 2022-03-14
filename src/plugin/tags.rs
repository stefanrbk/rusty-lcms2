use crate::plugin::*;

#[derive(Copy, Clone)]
pub struct TagListItem {
    pub sig: Signature,
    pub desc: TagDescriptor,
}
impl TagListItem {
    pub const DEFAULT: Self = Self {
        sig: Signature::new(b"    "),
        desc: TagDescriptor::DEFAULT,
    };
}

#[derive(Copy, Clone)]
pub struct TagDescriptor {
    /// If this tag needs to be an array, how many elements should keep
    pub element_count: u32,

    pub supported_types: &'static [Signature],

    pub decide_type: Option<DecideType>,
}
impl TagDescriptor {
    pub const DEFAULT: Self = Self {
        element_count: 0,
        supported_types: EMPTY_SIGNATURES,
        decide_type: None,
    };
}

const EMPTY_SIGNATURES: &[Signature] = &[Signature::new(b"    "); 0];
