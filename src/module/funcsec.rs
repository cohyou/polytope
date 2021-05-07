use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};
use crate::valtype::ValType;
use super::idx::{Typeidx, read_typeidx};
use super::expr::Expr;
use super::codesec::Code;


pub(super) struct Func {
    #[allow(dead_code)] tp: Typeidx,
    locals: Vec<ValType>,
    body: Expr,
}

impl Func {
    #[allow(non_snake_case)] 
    pub(super) fn setCode(&mut self, code: &Code) {
        self.locals = code.locals();
        self.body = code.body();
    }
}

pub(super) fn read_funcsec(reader: &mut impl Read) -> Vec<Func> {
    // prefixはsection number 3
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, |reader| {
        Func {
            tp: read_typeidx(reader),
            locals: vec![],
            body: Expr::default(),
        }
    })
}
