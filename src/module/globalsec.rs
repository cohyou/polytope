use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};
use super::globaltype::{GlobalType, read_globaltype};
use super::expr::{Expr, read_expr};

pub(super) struct Global {
    tp: GlobalType,
    init: Expr,
}


pub(super) fn read_globalsec(reader: &mut impl Read) -> Vec<Global> {
    // prefixはsection number 6
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_global)
}

fn read_global(reader: &mut impl Read) -> Global {
    let globaltype = read_globaltype(reader);
    let expr = read_expr(reader);
    Global { tp: globaltype, init: expr }
}