use std::io::Read;
use super::valtype::{ValType, read_valtype};
use super::util::read_vec;

pub(super) fn read_resulttype(reader: &mut impl Read) -> Vec<ValType> {
    read_vec(reader, read_valtype)
}

#[test]
fn test_read_resulttype() {
    use std::io::BufReader;

    let data: [u8; 6] = [
        4,
        0x7D, 0x7C, 0x7F, 0x7E, 0x7D,
    ];
    let mut reader = BufReader::new(data.as_ref());
    let correct = vec![ValType::F32, ValType::F64, ValType::I32, ValType::I64];
    assert_eq!(read_resulttype(&mut reader), correct);
}