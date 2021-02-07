use std::io::Read;
use super::valtype::ValType;
use super::resulttype::read_resulttype;

pub(super) type FuncType = (Vec<ValType>, Vec<ValType>);

pub(super) fn read_functype(reader: &mut impl Read) -> FuncType {
    // 0x60がprefix
    let param_types = read_resulttype(reader);
    let ret_types = read_resulttype(reader);
    (param_types, ret_types)
}
