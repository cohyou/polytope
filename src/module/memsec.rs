use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};
use super::memtype::{MemType, read_memtype};

pub(super) struct Mem(MemType);

pub(super) fn read_memsec(reader: &mut impl Read) -> Vec<Mem> {
    // prefixはsection number 5
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_mem)
}

fn read_mem(reader: &mut impl Read) -> Mem {
    Mem(read_memtype(reader))
}