use std::io::Read;

pub(super) enum ElemType { FuncRef, }

pub(super) fn read_elemtype(reader: &mut impl Read) -> ElemType {
    if let Some(Ok(byte)) = reader.bytes().next() {
        if byte == 0x70 {
            ElemType::FuncRef
        } else {
            panic!("invalid on read_elemtype");
        }
    } else {
        panic!("invalid on read_elemtype");
    }   
}
