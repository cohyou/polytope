use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};
use super::tabletype::{TableType, read_tabletype};

pub(super) struct Table(TableType);

pub(super) fn read_tablesec(reader: &mut impl Read) -> Vec<Table> {
    // prefixはsection number 4
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_table)
}

fn read_table(reader: &mut impl Read) -> Table {
    let tabletype = read_tabletype(reader);
    Table(tabletype)
}