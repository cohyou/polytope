use std::io::Read;
use super::limits::{Limits, read_limits};
use super::elemtype::{ElemType, read_elemtype};

pub(super) struct TableType(Limits, ElemType);

pub(super) fn read_tabletype(reader: &mut impl Read) -> TableType {
    let limits = read_limits(reader);
    let elem_type = read_elemtype(reader);
    TableType(limits, elem_type)
}

