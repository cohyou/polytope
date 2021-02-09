use std::io::Read;
use super::byte::Byte;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValType {
    I32, I64, F32, F64,
}

fn byte_to_valtype(b :Byte) -> ValType {
    match b {
        0x7F => ValType::I32,
        0x7E => ValType::I64,
        0x7D => ValType::F32,
        0x7C => ValType::F64,
        _ => panic!("invalid on byte_to_valtype: {:x?}", b),
    }
}

pub(super) fn read_valtype(reader: &mut impl Read) -> ValType {
    let byte = reader.bytes().next();
    if let Some(Ok(b)) = byte {
        byte_to_valtype(b)
    } else {
        panic!("invalid on read_valtype");
    }
}
