use std::fmt::{Debug, Display};

use compactor::huffman::*;
use compactor::*;

fn main() {
    // let test_value = "aaabbc";

    // let (test_encode, test_tree) = encode(&test_value);

    // println!("encoding: {:?}", test_encode);
    // println!("tree: {:?}", test_tree);

    // let test_decode = decode(test_encode, &test_tree);

    // println!("decoded: {:?}", test_decode);
    // assert_eq!(test_value, test_decode)
    let mut bin = BinaryStack::new();
    println!("{:?}", bin);

    for i in 0..10 {
        bin.push_bit(i % 2 == 1);
        println!("{:?}", bin);
    }

    // let mut test = 64u8;

    // test <<= 1;

    // println!("{}", test);
}
#[allow(dead_code)]
// #[derive(Debug)]
pub struct BinaryStack {
    pointer: (usize, u8),
    bytes: Vec<u8>,
}

impl BinaryStack {
    pub fn new() -> Self {
        Self {
            pointer: (0, 128),
            bytes: (vec![0]),
        }
    }

    pub fn push_bit(&mut self, bit: bool) {
        let byte_pntr = self.pointer.0;
        let bit_pntr = self.pointer.1;
        self.bytes[byte_pntr] |= bit_pntr * bit as u8;
        self.increment_pointer();
    }

    fn increment_pointer(&mut self) {
        // let overflow = self.pointer.1 == 128;
        // self.pointer.0 += overflow as usize; // byte
        // self.pointer.1 <<= 1; // bit
        // self.pointer.1 += overflow as u8;
        if self.pointer.1 == 1 {
            self.pointer.1 = 128;
            self.pointer.0 += 1;
            self.bytes.push(0u8);
        } else {
            self.pointer.1 >>= 1;
        }
    }
}

impl Debug for BinaryStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut byte_string = String::new();
        for byte in self.bytes.iter() {
            byte_string.push_str(&format!("{:08b}", byte))
        }
        write!(
            f,
            "BinaryStack {{pointer: ({},{:08b}), bytes: {} }}",
            self.pointer.0, self.pointer.1, byte_string
        )
    }
}

impl Display for BinaryStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut byte_string = String::new();
        for byte in self.bytes.iter() {
            byte_string.push_str(&format!("{:08b}", byte))
        }
        write!(f, "{}", byte_string)
    }
}
