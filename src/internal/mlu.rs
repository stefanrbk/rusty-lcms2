use std::collections::HashMap;

pub const NO_LANGUAGE: u16 = 0;
pub const NO_COUNTRY: u16 = 0;

pub const NO_LANGUAGE_OR_COUNTRY: (u16, u16) = (NO_LANGUAGE, NO_COUNTRY);

pub type Mlu = HashMap<(u16, u16), String>;
