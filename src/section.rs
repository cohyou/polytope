// type Byte = u8;
use std::io::Read;
use crate::core::{ValType, FuncType, Module};

pub enum Section {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

type Byte = u8;
pub fn id_to_section(id: Byte) -> Section {
    match id {
        0 => Section::Custom,
        1 => Section::Type,
        2 => Section::Import,
        3 => Section::Function,
        4 => Section::Table,
        5 => Section::Memory,
        6 => Section::Global,
        7 => Section::Export,
        8 => Section::Start,
        9 => Section::Element,
        10 => Section::Code,
        11 => Section::Data,
        _ => panic!("invalid on id_to_section")
    }
}

pub fn read_u32_from_leb128(reader: &mut impl Read) -> u32 {
    let mut acc: u32 = 0;
    let mut count: u8 = 0;
    for byte in reader.bytes() {
        if let Ok(b) = byte {
            let val: u32 = (b & 0b01111111) as u32;
            let shifted_val = val << (7 * count);
            acc += shifted_val as u32;
            count += 1;
            if b < 0b10000000 { break; }
        } else {
            break;
        }
    }
    acc
}

pub fn read_name(reader: &mut impl Read) -> String {
    let length = read_u32_from_leb128(reader);
    let mut buffer = String::new();
    let mut handle = reader.take(length as u64);
    let _ = handle.read_to_string(&mut buffer);
    buffer
}

fn read_valtype(reader: &mut impl Read) -> ValType {
    let byte = reader.bytes().next();
    if let Some(Ok(b)) = byte {
        byte_to_valtype(b)
    } else {
        panic!("invalid on read_valtype");
    }
}

fn byte_to_valtype(b :Byte) -> ValType {
    match b {
        0x7F => ValType::i32,
        0x7E => ValType::i64,
        0x7D => ValType::f32,
        0x7C => ValType::f64,
        _ => panic!("invalid on byte_to_valtype: {:x?}", b),
    }
}

pub fn read_resulttype(reader: &mut impl Read) -> Vec<ValType> {
    read_vec(reader, read_valtype)
}

fn read_functype(reader: &mut impl Read) -> FuncType {
    // 0x60がprefix
    let param_types = read_resulttype(reader);
    let ret_types = read_resulttype(reader);
    (param_types, ret_types)
}

fn read_vec<T: Read, R>(reader: &mut T, f: fn(reader: &mut T) -> R) -> Vec<R> {
    let length = read_u32_from_leb128(reader);
    (0..length).map(|_| f(reader)).collect()
}

use std::io;
fn read_magic(reader: &mut impl Read) -> io::Result<()> {
    let magic: [u8; 4] = [0x00, 0x61, 0x73, 0x6D,];
    let mut buf: [u8; 4] = [0x00; 4];
    reader.read_exact(&mut buf)?;
    if buf == magic {
        Ok(())
    } else {
        panic!("invalid on read_magic");
    }
}

fn read_version(reader: &mut impl Read) -> io::Result<()> {
    let magic: [u8; 4] = [0x01, 0x00, 0x00, 0x00,];
    let mut buf: [u8; 4] = [0x00; 4];
    reader.read_exact(&mut buf)?;
    if buf == magic {
        Ok(())
    } else {
        panic!("invalid on read_magic");
    }
}

fn read_customsec(reader: &mut impl Read) {
    // prefixはsection number 0
    let length = read_u32_from_leb128(reader);
    let mut bytes = reader.bytes();
    let _ = (0..length).map(|_| bytes.next());
}

fn read_typesec(reader: &mut impl Read) -> Vec<FuncType> {
    // prefixはsection number 1
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_functype)
}

pub fn read_module(reader: &mut impl Read) -> io::Result<Module> {
    let mut module = Module::default();

    read_magic(reader)?;
    read_version(reader)?;
    while let Some(Ok(section_id)) = reader.bytes().next() {
        match id_to_section(section_id) {
            Section::Custom => read_customsec(reader),
            Section::Type => {
                module.types = read_typesec(reader);
            },
            Section::Import => (),
            Section::Function => (),
            Section::Table => (),
            Section::Memory => (),
            Section::Global => (),
            Section::Export => (),
            Section::Start => (),
            Section::Element => (),
            Section::Code => (),
            Section::Data => (),
        }
    }

    Ok(module)
}
