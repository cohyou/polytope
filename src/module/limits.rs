use std::io::Read;
use crate::util::read_u32_from_leb128;

pub struct Limits {
    #[allow(dead_code)] min: u32,
    #[allow(dead_code)] max: Option<u32>,
}

impl Limits {
    pub fn new1(min: u32) -> Limits {
        Limits { min: min, max: None }
    }
    pub fn new2(min: u32, max: u32) -> Limits {
        Limits { min: min, max: Some(max) }
    }
}

pub(super) fn read_limits(reader: &mut impl Read) -> Limits {
    if let Some(Ok(byte)) = reader.bytes().next() {
        match byte {
            0x00 => {
                // only min
                let min_size = read_u32_from_leb128(reader);  // min
                Limits::new1(min_size)
            },  
            0x01 => {
                // min and max
                let min_size = read_u32_from_leb128(reader);  // min
                let max_size = read_u32_from_leb128(reader);  // max
                Limits::new2(min_size, max_size)
            }, 
            _ => panic!("invalid on read_limits"),
        }
    } else {
        panic!("invalid on read_limits");
    }
}


