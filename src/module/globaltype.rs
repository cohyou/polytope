use std::io::Read;
use crate::valtype::{ValType, read_valtype};


pub(super) struct GlobalType(ValType, Mut);
enum Mut { Const, Var }


pub(super) fn read_globaltype(reader: &mut impl Read) -> GlobalType {
    let valtype = read_valtype(reader);
    let mutablilty = read_mut(reader);
    GlobalType(valtype, mutablilty)
}

fn read_mut(reader: &mut impl Read) -> Mut {
    if let Some(Ok(b)) = reader.bytes().next() {
        match b {
            0x00 => Mut::Const,
            0x01 => Mut::Var,
            _ => panic!("invalid on read_mut"),
        }
    } else {
        unimplemented!()
    }   
}
