use crate::CmsSignature;

pub mod tag;
pub mod tag_type;

pub const MAGIC_NUMBER: CmsSignature = CmsSignature::new(b"acsp");
pub const LCMS_SIGNATURE: CmsSignature = CmsSignature::new(b"lcms");

pub mod color_space;
pub mod platform;
pub mod profile_class;
pub mod technology;

// Reference gamut
pub const PERCEPTUAL_REFERENCE_MEDIUM_GAMUT: CmsSignature = CmsSignature::new(b"prmg");

// For SigColorimetricIntentImageStateTag
pub const SCENE_COLORIMETRY_ESTIMATES: CmsSignature = CmsSignature::new(b"scoe");
pub const SCENE_APPEARANCE_ESTIMATES: CmsSignature = CmsSignature::new(b"sape");
pub const FOCAL_PLANE_COLORIMETRY_ESTIMATES: CmsSignature = CmsSignature::new(b"fpce");
pub const REFLECTION_HARDCOPY_ORIGINAL_COLORIMETRY: CmsSignature = CmsSignature::new(b"rhoc");
pub const REFLECTION_PRINT_OUTPUT_COLORIMETRY: CmsSignature = CmsSignature::new(b"rpoc");

pub mod stage;
pub mod curve_segment {
    use crate::CmsSignature;

    pub const FORMULA: CmsSignature = CmsSignature::new(b"parf");
    pub const SAMPLED: CmsSignature = CmsSignature::new(b"samf");
    pub const SEGMENTED: CmsSignature = CmsSignature::new(b"curf");
}
pub const STATUS_A: CmsSignature = CmsSignature::new(b"StaA");
pub const STATUS_E: CmsSignature = CmsSignature::new(b"StaE");
pub const STATUS_I: CmsSignature = CmsSignature::new(b"StaI");
pub const STATUS_T: CmsSignature = CmsSignature::new(b"StaT");
pub const STATUS_M: CmsSignature = CmsSignature::new(b"StaM");
pub const DN: CmsSignature = CmsSignature::new(b"DN  ");
pub const DNP: CmsSignature = CmsSignature::new(b"DN P");
pub const DNN: CmsSignature = CmsSignature::new(b"DNN ");
pub const DNNP: CmsSignature = CmsSignature::new(b"DNNP");
