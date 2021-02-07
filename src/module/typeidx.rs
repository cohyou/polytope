use std::io::Read;
use crate::util::read_u32_from_leb128;

pub(super) type Typeidx = u32;

pub(super) fn read_typeidx(reader: &mut impl Read) -> Typeidx {
    read_u32_from_leb128(reader)
}
