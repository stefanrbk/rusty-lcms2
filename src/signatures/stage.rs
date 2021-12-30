use crate::CmsSignature;

pub const CURVE_SET_ELEM_TYPE: CmsSignature = CmsSignature::new(b"cvst");
pub const MATRIX_ELEM_TYPE: CmsSignature = CmsSignature::new(b"matf");
pub const C_LUT_ELEM_TYPE: CmsSignature = CmsSignature::new(b"clut");

pub const B_ACS_ELEM_TYPE: CmsSignature = CmsSignature::new(b"bACS");
pub const E_ACS_ELEM_TYPE: CmsSignature = CmsSignature::new(b"eACS");

// Custom from here, not in the ICC Spec
pub const XYZ_TO_LAB_ELEM_TYPE: CmsSignature = CmsSignature::new(b"l2x ");
pub const LAB_TO_XYZ_ELEM_TYPE: CmsSignature = CmsSignature::new(b"x2l ");
pub const NAMED_COLOR_ELEM_TYPE: CmsSignature = CmsSignature::new(b"ncl ");
pub const LAB_V2_TO_V4: CmsSignature = CmsSignature::new(b"2 4 ");
pub const LAB_V4_TO_V2: CmsSignature = CmsSignature::new(b"4 2 ");

// Identities
pub const IDENTITY_ELEM_TYPE: CmsSignature = CmsSignature::new(b"idn ");

// Float to floatPCS
pub const LAB_TO_FLOAT_PCS: CmsSignature = CmsSignature::new(b"d2l ");
pub const FLOAT_PCS_TO_LAB: CmsSignature = CmsSignature::new(b"l2d ");
pub const XYZ_TO_FLOAT_PCS: CmsSignature = CmsSignature::new(b"d2x ");
pub const FLOAT_PCS_TO_XYZ: CmsSignature = CmsSignature::new(b"x2d ");
pub const CLIP_NEGATIVES_ELEM_TYPE: CmsSignature = CmsSignature::new(b"clp ");
