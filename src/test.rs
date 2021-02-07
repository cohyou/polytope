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
}

