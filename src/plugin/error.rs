use crate::plugin::Context;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
#[non_exhaustive]
pub enum ErrorCode {
    Undefined = 0,
    File = 1,
    Range = 2,
    Internal = 3,
    Null = 4,
    Read = 5,
    Seek = 6,
    Write = 7,
    UnknownExtension = 8,
    ColorSpaceCheck = 9,
    AlreadyDefined = 10,
    BadSignature = 11,
    CorruptionDetected = 12,
    NotSuitable = 13,
}

pub type LogErrorHandlerFunction = fn(ErrorCode, &str);

pub fn default_log_error_handler_function(_code: ErrorCode, _text: &str) {}

pub fn signal_error(context: &mut Context, code: ErrorCode, text: &str) {
    (context.get_log_error_chunk().handler)(code, text);
}
