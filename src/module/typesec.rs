use std::io::Read;
use crate::functype::{FuncType, read_functype};
use crate::util::{read_u32_from_leb128, read_vec};

pub(super) fn read_typesec(reader: &mut impl Read) -> Vec<FuncType> {
    // prefixはsection number 1
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_functype)
}
