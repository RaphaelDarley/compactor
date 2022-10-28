use std::fmt::{Debug, Display};

use compactor::huffman::*;
use compactor::BinararyStack::BinaryStack;
use compactor::*;
use rand::Rng;

fn main() {
    // let test_value = "aaabbc";

    // let (test_encode, test_tree) = encode(&test_value);

    // println!("encoding: {:?}", test_encode);
    // println!("tree: {:?}", test_tree);

    // let test_decode = decode(test_encode, &test_tree);

    // println!("decoded: {:?}", test_decode);
    // assert_eq!(test_value, test_decode)
    let mut bin = BinaryStack::new();
    // println!("{:?}", bin);

    for i in 0..10 {
        bin.push_bit(i % 2 == 1);
        // println!("{}", bin);
        // println!("{:?}", bin.read());
    }

    bin.reset_pointer();
    println!("{:?}", bin);

    for bit in bin {
        println!("{:?}", bit);
    }

    // let mut test = 64u8;

    // test <<= 1;

    // println!("{}", test);
}
