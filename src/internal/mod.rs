use std::io::*;

mod types;

pub const eof_error: Error = Error::new(
        ErrorKind::UnexpectedEof,
        "Can't read from buffer. Unexpected EOF.",
);

pub const invalid_data_error: Error =
    Error::new(
        ErrorKind::InvalidData,
        "Encountered invalid data.",
);

pub const MATRIX_DET_TOLERANCE: f64 = 0.0001;
