use crate::CmsSignature;

pub const INPUT_CLASS: CmsSignature = CmsSignature::new(b"scnr");
pub const DISPLAY_CLASS: CmsSignature = CmsSignature::new(b"mntr");
pub const OUTPUT_CLASS: CmsSignature = CmsSignature::new(b"prtr");
pub const LINK_CLASS: CmsSignature = CmsSignature::new(b"link");
pub const ABSTRACT_CLASS: CmsSignature = CmsSignature::new(b"abst");
pub const COLOR_SPACE_CLASS: CmsSignature = CmsSignature::new(b"spac");
pub const NAMED_COLOR_CLASS: CmsSignature = CmsSignature::new(b"nmcl");
