use std::io::Read;

use super::limits::{Limits, read_limits};


pub(crate) struct MemType(Limits);


pub(super) fn read_memtype(reader: &mut impl Read) -> MemType {
    MemType(read_limits(reader))
}