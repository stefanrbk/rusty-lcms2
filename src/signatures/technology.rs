//! ICC Technology tag
use crate::Signature;

pub const DIGITAL_CAMERA: Signature = Signature::new(b"dcam");
pub const FILM_SCANNER: Signature = Signature::new(b"fscn");
pub const REFLECTIVE_SCANNER: Signature = Signature::new(b"rscn");
pub const INKJET_PRINTER: Signature = Signature::new(b"ijet");
pub const THERMAL_WAX_PRINTER: Signature = Signature::new(b"twax");
pub const ELECTROPHOTOGRAPHIC_PRINTER: Signature = Signature::new(b"epho");
pub const ELECTROSTATIC_PRINTER: Signature = Signature::new(b"esta");
pub const DYE_SUBLIMATION_PRINTER: Signature = Signature::new(b"dsub");
pub const PHOTOGRAPHIC_PAPER_PRINTER: Signature = Signature::new(b"rpho");
pub const FILM_WRITER: Signature = Signature::new(b"fprn");
pub const VIDEO_MONITOR: Signature = Signature::new(b"vidm");
pub const VIDEO_CAMERA: Signature = Signature::new(b"vidc");
pub const PROJECTION_TELEVISION: Signature = Signature::new(b"pjtv");
pub const CRT_DISPLAY: Signature = Signature::new(b"CRT ");
pub const PM_DISPLAY: Signature = Signature::new(b"PMD ");
pub const AM_DISPLAY: Signature = Signature::new(b"AMD ");
pub const PHOTO_CD: Signature = Signature::new(b"KPCD");
pub const PHOTO_IMAGE_SETTER: Signature = Signature::new(b"imgs");
pub const GRAVURE: Signature = Signature::new(b"grav");
pub const OFFSET_LITHOGRAPHY: Signature = Signature::new(b"offs");
pub const SILKSCREEN: Signature = Signature::new(b"silk");
pub const FLEXOGRAPHY: Signature = Signature::new(b"flex");
pub const MOTION_PICTURE_FILM_SCANNER: Signature = Signature::new(b"mpfs");
pub const MOTION_PICTURE_FILM_RECORDER: Signature = Signature::new(b"mpfr");
pub const DIGITAL_MOTION_PICTURE_CAMERA: Signature = Signature::new(b"dmpc");
pub const DIGITAL_CINEMA_PROJECTOR: Signature = Signature::new(b"dcpj");
