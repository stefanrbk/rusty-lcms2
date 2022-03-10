mod types;

pub const MATRIX_DET_TOLERANCE: f64 = 0.0001;

pub struct MluEntry {
    pub language: u16,
    pub country: u16,
    pub value: String,
}

pub struct Mlu<'a>(&'a [MluEntry]);
