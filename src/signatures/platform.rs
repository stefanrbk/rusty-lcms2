use crate::CmsSignature;

pub const MACINTOSH: CmsSignature = CmsSignature::new(b"APPL");
pub const MICROSOFT: CmsSignature = CmsSignature::new(b"MSFT");
pub const SOLARIS: CmsSignature = CmsSignature::new(b"SUNW");
pub const SGI: CmsSignature = CmsSignature::new(b"SGI ");
pub const TALIGENT: CmsSignature = CmsSignature::new(b"TGNT");
pub const UNICES: CmsSignature = CmsSignature::new(b"*nix");
