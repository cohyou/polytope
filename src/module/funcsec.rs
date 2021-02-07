use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};
use super::typeidx::{Typeidx, read_typeidx};

pub(super) fn read_funcsec(reader: &mut impl Read) -> Vec<Typeidx> {
    // prefixはsection number 3
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_typeidx)
}
