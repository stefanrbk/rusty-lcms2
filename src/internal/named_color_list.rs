use crate::*;
use std::io::Result;
use std::ops::Index;

#[derive(Clone)]
pub struct NamedColor {
    pub name: String,
    pub pcs: [u16; 3],
    pub device_colorant: [u16; MAX_CHANNELS],
}

#[derive(Clone)]
pub struct NamedColorList {
    num_colors: u32,
    colorant_count: u32,
    prefix: String,
    suffix: String,
    list: Vec<NamedColor>,
}

impl NamedColorList {
    pub fn append(
        &mut self,
        name: String,
        pcs: Option<[u16; 3]>,
        colorant: Option<[u16; MAX_CHANNELS]>,
    ) {
        let colorant = colorant.unwrap_or([0u16; MAX_CHANNELS]);
        let pcs = pcs.unwrap_or([0u16; 3]);

        let name = match name {
            n if n.len() >= MAX_PATH => n[..MAX_PATH].to_string(),
            _ => name,
        };

        self.list.push(NamedColor {
            name: name,
            pcs: pcs,
            device_colorant: colorant,
        })
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }

    pub fn get(&self, color_number: usize) -> Option<NamedColor> {
        match self.list.get(color_number) {
            None => None,
            Some(value) => Some(value.clone()),
        }
    }
}
