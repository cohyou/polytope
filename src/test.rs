use std::io::BufReader;
use crate::section::read_u32_from_leb128;

#[allow(overflowing_literals)]
pub fn test_rotate_shift() {
    // let n = 0xaa00000000006e1u64;
    // let n = 0x1234567890ABCDEFu64;
    // // let m = 0x6e10aa;
    // println!("{:x}", n.rotate_left(8));
    // assert_eq!(n.rotate_left(1), m);
    let o = 0b11110000u8;
    assert_eq!(o.rotate_left(3), 0b10000111);
    println!("{:b}", o.rotate_left(3));
    assert_eq!(o << 3, 0b10000000);
    
    let p = 0b10000000i8;
    println!("{:8b}", p.rotate_right(1));
    println!("{:8b}", p >> 1);
    assert_eq!(p.rotate_right(1), 0b01000000);
    assert_eq!(p >> 1, 0b11000000);

    // let data: [u8; 4] = [0b11001100, 0b01000011, 86, 120];
    let data: [u8; 4] = [0xE5, 0x8E, 0x26, 120];
    let mut reader = BufReader::new(data.as_ref());
    let res = read_u32_from_leb128(&mut reader);
    println!("res: {:b}", res);
    println!("res: {}", res);
}

#[test]
pub fn test_read_u32_from_leb128() {
    let data: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    let mut reader = BufReader::new(data.as_ref());
    let res = read_u32_from_leb128(&mut reader);
    println!("{:x?}", res);
}

#[test]
fn test_read_name() {
    let data: [u8; 10] = [
        6,
        0xe3, 0x81, 0x86, // う
        0xe3, 0x81, 0xa9, // ど
        0xe3, 0x82, 0x93, // ん
    ];
    let mut reader = BufReader::new(data.as_ref());
    assert_eq!(read_name(&mut reader), "うど".to_string());
}

#[test]
fn test_read_resulttype() {
    let data: [u8; 6] = [
        4,
        0x7D, 0x7C, 0x7F, 0x7E, 0x7D,
    ];
    let mut reader = BufReader::new(data.as_ref());
    let correct = vec![ValType::f32, ValType::f64, ValType::i32, ValType::i64];
    assert_eq!(read_resulttype(&mut reader), correct);
}
