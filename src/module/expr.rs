use std::io::Read;

use super::instr::Instr;


pub(super) struct Expr(Vec<Instr>);

pub(super) fn read_expr(reader: &mut impl Read) -> Expr {
    Expr(vec![])
}