use std::io::Result;
use std::io::{Error, ErrorKind};

pub const NO_LANGUAGE: [u8; 2] = *b"\0\0";
pub const NO_COUNTRY: [u8; 2] = *b"\0\0";

#[derive(Clone)]
pub struct Mlu {
    list: Vec<MluEntry<u16>>,
    default: Option<MluEntry<u16>>,
}
#[derive(Clone)]
pub struct MluEntry<T = [u8; 2]>
where
    T: Sized + Clone,
{
    pub lang: T,
    pub cntry: T,
    pub string: String,
}

#[derive(Clone)]
pub struct MluResult<T = [u8; 2]>
where
    T: Sized + Clone,
{
    pub string: String,
    pub obtained_language: T,
    pub obtained_country: T,
}

fn str_to_u16(s: Option<[u8; 2]>) -> u16 {
    if let Some(s) = s {
        return (s[0]) as u16 | s[1] as u16;
    } else {
        return 0;
    }
}
fn str_from_u16(v: u16) -> [u8; 2] {
    [((v & 0xFF00) >> 8) as u8, (v & 0x00FF) as u8]
}

impl Mlu {
    pub const fn new() -> Mlu {
        Mlu {
            list: Vec::new(),
            default: None,
        }
    }
    pub fn set_ascii(
        &mut self,
        lang: Option<[u8; 2]>,
        cntry: Option<[u8; 2]>,
        ascii_string: String,
    ) -> Result<()> {
        // ASCII check!!
        for c in ascii_string.chars() {
            if !c.is_ascii() {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        }

        let lang = str_to_u16(lang);
        let cntry = str_to_u16(cntry);

        self.add(lang, cntry, ascii_string)
    }
    pub fn set_unicode(
        &mut self,
        lang: Option<[u8; 2]>,
        cntry: Option<[u8; 2]>,
        utf8_string: String,
    ) -> Result<()> {
        let lang = str_to_u16(lang);
        let cntry = str_to_u16(cntry);

        self.add(lang, cntry, utf8_string)
    }

    fn search_for_entry(&self, lang: u16, cntry: u16) -> Option<&String> {
        for v in self.list.iter() {
            if v.lang == lang && v.cntry == cntry {
                return Some(&v.string);
            }
        }
        None
    }
    fn add(&mut self, lang: u16, cntry: u16, s: String) -> Result<()> {
        match self.search_for_entry(lang, cntry) {
            Some(_) => Err(Error::from(ErrorKind::AlreadyExists)),
            None => {
                let value = MluEntry {
                    lang: lang,
                    cntry: cntry,
                    string: s.to_string(),
                };
                self.list.push(value.clone());
                if self.default.is_none() {
                    self.default = Some(value);
                }
                Ok(())
            }
        }
    }

    pub fn get_ascii(&self, lang: u16, cntry: u16) -> Result<Option<MluResult>> {
        let utf = self.get_unicode(lang, cntry);

        if let Some(value) = &utf {
            // ASCII check!!
            for c in value.string.chars() {
                if !c.is_ascii() {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
        }

        Ok(utf)
    }

    pub fn get_unicode(&self, lang: u16, cntry: u16) -> Option<MluResult> {
        match self.search_for_entry(lang, cntry) {
            Some(value) => Some(MluResult {
                string: value.to_string(),
                obtained_language: str_from_u16(lang),
                obtained_country: str_from_u16(cntry),
            }),
            None => {
                for v in self.list.iter() {
                    if v.lang == lang {
                        return Some(MluResult {
                            string: v.string.to_string(),
                            obtained_language: str_from_u16(v.lang),
                            obtained_country: str_from_u16(v.cntry),
                        });
                    }
                }
                if let Some(value) = &self.default {
                    Some(MluResult {
                        string: value.string.to_string(),
                        obtained_language: str_from_u16(value.lang),
                        obtained_country: str_from_u16(value.cntry),
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn get_translation(&self, lang: [u8; 2], cntry: [u8; 2]) -> Option<MluResult> {
        let lang = str_to_u16(Some(lang));
        let cntry = str_to_u16(Some(cntry));

        let utf = match self.get_unicode(lang, cntry) {
            None => return None,
            Some(value) => value
        };

        let obt_lang = utf.obtained_language;
        let obt_cntry = utf.obtained_country;

        Some(MluResult {
            string: utf.string,
            obtained_language: obt_lang,
            obtained_country: obt_cntry
        })
    }

    pub fn get_translation_count(&self) -> usize {
        self.list.len()
    }

    pub fn get_translation_codes(&self, index: usize) -> Option<MluEntry> {
        match self.list.get(index) {
            Some(entry) => Some(MluEntry {
                string: entry.string.to_string(),
                lang: str_from_u16(entry.lang),
                cntry: str_from_u16(entry.cntry),
            }),
            None => None
        }
    }
}
