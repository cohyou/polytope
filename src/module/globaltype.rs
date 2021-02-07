use std::io::Read;
use crate::valtype::{ValType, read_valtype};

enum Mut { Const, Var }
struct GlobalType(ValType, Mut);

fn read_mut(reader: &mut impl Read) -> Mut {
    Mut::Const
}

fn read_globaltype(reader: &mut impl Read) -> GlobalType {
    let valtype = read_valtype(reader);
    let mutablilty = read_mut(reader);
    GlobalType(valtype, mutablilty)
}