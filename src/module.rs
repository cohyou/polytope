use std::io::{self, Read};
use super::context::Context;

mod idx;
mod elemtype;
mod globaltype;
mod instr;

mod limits;
mod name;
mod expr;

mod memtype;
mod tabletype;

mod customsec;
mod typesec;
mod importsec;
mod funcsec;
mod tablesec;
mod memsec;
mod globalsec;
mod exportsec;
mod startsec;
mod elemsec;
mod codesec;
mod datasec;


use super::byte::Byte;
use super::functype::FuncType;
use customsec::read_customsec;
use typesec::read_typesec;
use importsec::{Import, read_importsec};
use idx::Typeidx;
use funcsec::read_funcsec;
use tablesec::{Table, read_tablesec};
use memsec::{Mem, read_memsec};
use globalsec::{Global, read_globalsec};
use exportsec::{Export, read_exportsec};
use startsec::{Start, read_startsec};
use elemsec::{Elem, read_elemsec};
use datasec::{Data, read_datasec};


#[derive(Default)]
pub(super) struct Module {
    types: Vec<FuncType>, 
    imports: Vec<Import>,
    funcs: Vec<Typeidx>,
    tables: Vec<Table>,
    mems: Vec<Mem>,
    globals: Vec<Global>,
    exports: Vec<Export>,
    start: Option<Start>,
    elem: Vec<Elem>,
    data: Vec<Data>,
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
            Section::Type => { module.types = read_typesec(reader); },
            Section::Import => { module.imports = read_importsec(reader) },
            Section::Function => { module.funcs = read_funcsec(reader) },
            Section::Table => { module.tables = read_tablesec(reader) },
            Section::Memory => { module.mems = read_memsec(reader) },
            Section::Global => { module.globals = read_globalsec(reader) },
            Section::Export => { module.exports = read_exportsec(reader) },
            Section::Start => { module.start = Some(read_startsec(reader)) },
            Section::Element => { module.elem = read_elemsec(reader) },
            Section::Code => (),
            Section::Data => { module.data = read_datasec(reader) },
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