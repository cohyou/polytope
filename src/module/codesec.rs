use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};
use crate::valtype::{ValType, read_valtype};
use super::expr::{Expr, read_expr};


pub(super) struct Code {
    size: u32,
    locals: Vec<Locals>,
    body: Expr,
}

// #[derive(Copy, Clone)]
struct Locals(u32, ValType);


impl Code {
    pub(super) fn locals(&self) -> Vec<ValType> {
        let mut res = vec![];
        for locals in &self.locals {
            res.extend(std::iter::repeat(locals.1).take(locals.0 as usize));
        }
        res
    }
    pub(super) fn body(&self) -> Expr { self.body.clone() }
}

pub(super) fn read_codesec(reader: &mut impl Read) -> Vec<Code> {
    // prefixはsection number 10
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_code)
}

fn read_code(reader: &mut impl Read) -> Code {
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);

    let length = read_u32_from_leb128(&mut handle);
    let mut handle = reader.take(length as u64);
    let locals_vec = read_vec(&mut handle, read_locals);
    let expr = read_expr(reader);

    Code { size: length, locals: locals_vec, body: expr }
}

fn read_locals(reader: &mut impl Read) -> Locals {
    let n = read_u32_from_leb128(reader);
    let valtype = read_valtype(reader);
    Locals(n, valtype)
}