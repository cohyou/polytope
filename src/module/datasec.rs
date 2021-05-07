use std::io::Read;

use crate::byte::Byte;
use crate::util::{read_u32_from_leb128, read_vec};
use super::idx::{Memidx, read_memidx};
use super::expr::{Expr, read_expr};


pub(super) struct Data {
    #[allow(dead_code)] data: Memidx,
    #[allow(dead_code)] offset: Expr,
    #[allow(dead_code)] init: Vec<Byte>,
}


pub(super) fn read_datasec(reader: &mut impl Read) -> Vec<Data> {
    // prefixはsection number 11
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_data)
}

fn read_data(reader: &mut impl Read) -> Data {
    let memidx = read_memidx(reader);
    let expr = read_expr(reader);
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    let init = read_vec(&mut handle, |reader| reader.bytes().next().unwrap().unwrap());

    Data {
        data: memidx,
        offset: expr,
        init: init,
    }
}