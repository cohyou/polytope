use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};

use super::idx::{Funcidx, Tableidx, Memidx, Globalidx};
use super::idx::{read_funcidx, read_tableidx, read_memidx, read_globalidx};
use super::name::{Name, read_name};


pub(super) struct Export {
    #[allow(dead_code)] name: Name,
    #[allow(dead_code)] desc: ExportDesc,
}

enum ExportDesc {
    Func(Funcidx),
    Table(Tableidx),
    Mem(Memidx),
    Global(Globalidx),
}


pub(super) fn read_exportsec(reader: &mut impl Read) -> Vec<Export> {
    // prefixはsection number 7
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_export)
}

fn read_export(reader: &mut impl Read) -> Export {
    let name_identifier = read_name(reader);
    let exportdesc = read_exportdesc(reader);
    Export {
        name: name_identifier,
        desc: exportdesc,
    }
}

fn read_exportdesc(reader: &mut impl Read) -> ExportDesc {
    if let Some(Ok(byte)) = reader.bytes().next() {
        match byte {
            0x00 => { return read_exportdesc_func(reader) },
            0x01 => { return read_exportdesc_tabletype(reader) },
            0x02 => { return read_exportdesc_memtype(reader) },
            0x03 => { return read_exportdesc_globaltype(reader) },
            _ => panic!("invalid on read_exportdesc"),
        }
    }

    unimplemented!()
}

fn read_exportdesc_func(reader: &mut impl Read) -> ExportDesc {
    ExportDesc::Func(read_funcidx(reader))
}

fn read_exportdesc_tabletype(reader: &mut impl Read) -> ExportDesc {
    ExportDesc::Table(read_tableidx(reader))
}

fn read_exportdesc_memtype(reader: &mut impl Read) -> ExportDesc {
    ExportDesc::Mem(read_memidx(reader))
}

fn read_exportdesc_globaltype(reader: &mut impl Read) -> ExportDesc {
    ExportDesc::Global(read_globalidx(reader))
}

