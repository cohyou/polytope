use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};


pub(super) type Typeidx = u32;
pub(super) type Funcidx = u32;
pub(super) type Tableidx = u32;
pub(super) type Memidx = u32;
pub(super) type Globalidx = u32;
pub(super) type Labelidx = u32;
pub(super) type Localidx = u32;


pub(super) fn read_typeidx(reader: &mut impl Read) -> Typeidx {
    read_u32_from_leb128(reader)
}

pub(super) fn read_funcidx(reader: &mut impl Read) -> Funcidx {
    read_u32_from_leb128(reader)
}

pub(super) fn read_tableidx(reader: &mut impl Read) -> Tableidx {
    read_u32_from_leb128(reader)
}

pub(super) fn read_memidx(reader: &mut impl Read) -> Typeidx {
    read_u32_from_leb128(reader)
}

pub(super) fn read_globalidx(reader: &mut impl Read) -> Globalidx {
    read_u32_from_leb128(reader)
}

pub(super) fn read_labelidx(reader: &mut impl Read) -> Labelidx {
    read_u32_from_leb128(reader)
}

pub(super) fn read_localidx(reader: &mut impl Read) -> Localidx {
    read_u32_from_leb128(reader)
}

pub(super) fn read_funcindices(reader: &mut impl Read) -> Vec<Funcidx> {
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_funcidx)
}

pub(super) fn read_labelindices(reader: &mut impl Read) -> Vec<Labelidx> {
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_labelidx)
}