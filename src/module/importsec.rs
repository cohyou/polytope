use std::io::Read;
use super::typeidx::Typeidx;
use super::tabletype::TableType;
use super::memtype::MemType;
use super::tabletype::read_tabletype;
use super::limits::read_limits;
use super::typeidx::read_typeidx;

pub(super) enum ImportDesc {
    Func(Typeidx),
    Table(TableType),
    Mem(MemType),
    Global,
}

fn read_importdesc(reader: &mut impl Read) -> ImportDesc {
    if let Some(Ok(byte)) = reader.bytes().next() {
        match byte {
            0x00 => { return read_importdesc_func(reader) },
            0x01 => { return read_importdesc_tabletype(reader) },
            0x02 => { return read_importdesc_memtype(reader) },
            0x03 => { return read_importdesc_globaltype(reader) },
            _ => panic!("invalid on read_importdesc"),
        }
    }

    unimplemented!()
}

fn read_importdesc_func(reader: &mut impl Read) -> ImportDesc {
    let typeidx = read_typeidx(reader);
    ImportDesc::Func(typeidx)
}

fn read_importdesc_tabletype(reader: &mut impl Read) -> ImportDesc {
    ImportDesc::Table(read_tabletype(reader))
}

fn read_importdesc_memtype(reader: &mut impl Read) -> ImportDesc {
    let limits = read_limits(reader);
    ImportDesc::Mem(MemType::new(limits))
}

fn read_importdesc_globaltype(reader: &mut impl Read) -> ImportDesc {
    ImportDesc::Global
}

