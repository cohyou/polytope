use std::io::{self, Read};
use super::context::Context;

mod typeidx;
mod elemtype;
mod globaltype;

mod limits;
mod name;

mod memtype;
mod tabletype;

mod customsec;
mod typesec;
mod importsec;


use super::byte::Byte;
use super::functype::FuncType;
use customsec::read_customsec;
use typesec::read_typesec;
use importsec::Import;

#[derive(Default)]
pub(super) struct Module {
    types: Vec<FuncType>, 
    imports: Vec<Import>,
}

enum Section {
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


pub(super) fn read_module(reader: &mut impl Read) -> io::Result<Module> {
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

pub(super) fn validate_module(module: Module) {
    let context = Context::new(module.types);
}

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

fn id_to_section(id: Byte) -> Section {
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