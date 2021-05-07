use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};
use super::idx::{Funcidx, Tableidx};
use super::idx::{read_tableidx, read_funcindices};
use super::expr::{Expr, read_expr};


pub(super) struct Elem {
    #[allow(dead_code)] table: Tableidx,
    #[allow(dead_code)] offset: Expr,
    #[allow(dead_code)] init: Vec<Funcidx>,
}

pub(super) fn read_elemsec(reader: &mut impl Read) -> Vec<Elem> {
    // prefixはsection number 9
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_elem)
}

fn read_elem(reader: &mut impl Read) -> Elem {
    let tableidx = read_tableidx(reader);
    let expr = read_expr(reader);
    let init = read_funcindices(reader);
    Elem {
        table: tableidx,
        offset: expr,
        init: init,
    }
}