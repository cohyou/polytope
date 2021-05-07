use std::io::Read;
use crate::byte::Byte;
use super::instr::{Instr, read_instr};


#[derive(Default, Clone)]
pub(super) struct Expr(pub(super) Vec<Instr>);

pub(super) fn read_expr(reader: &mut impl Read) -> Expr {
    read_instrs(reader, 0x0B)
}

#[allow(dead_code)]
pub(super) fn read_expr_else(reader: &mut impl Read) -> Expr {
    read_instrs(reader, 0x05)
}

fn read_instrs(reader: &mut impl Read, stop: Byte) -> Expr {
    let mut instrs = vec![];

    loop {
        if let Some(Ok(b)) = reader.bytes().next() {
            if b == stop { break; }  // end

            let instr = read_instr(b, reader);
            instrs.push(instr);
        } else {
            break;
        }
    }

    Expr(instrs)
}