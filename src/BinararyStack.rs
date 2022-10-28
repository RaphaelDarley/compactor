use std::fmt::{Debug, Display};

pub struct BinaryStack {
    pntr: BSPointer,
    bytes: Vec<u8>,
}

struct BSPointer {
    byte_pntr: usize,
    bit_mask: u8,
}

impl BSPointer {
    fn from(byte_pntr: usize, bit_mask: u8) -> Self {
        Self {
            byte_pntr,
            bit_mask,
        }
    }
}

// init methods
impl BinaryStack {
    pub fn new() -> Self {
        Self {
            pntr: BSPointer::from(0, 128),
            bytes: (vec![0]),
        }
    }
}

// read methods
impl BinaryStack {
    pub fn read(&self) -> Option<bool> {
        if self.bytes.len() <= self.pntr.byte_pntr {
            None
        } else {
            let byte = self.bytes[self.pntr.byte_pntr];
            Some((byte & self.pntr.bit_mask) != 0)
        }
    }

    // //offset is in bits
    // pub fn read_prev(&self) -> bool {
    //     let byte = self.bytes[self.pntr.byte_pntr];
    //     (byte & (self.pntr.bit_mask << 1)) != 0
    // }
}

//writing methods
impl BinaryStack {
    pub fn push_bit(&mut self, bit: bool) {
        self.bytes[self.pntr.byte_pntr] |= self.pntr.bit_mask * bit as u8;
        self.increment_pointer_and_expand();
    }

    fn increment_pointer_and_expand(&mut self) {
        if self.pntr.bit_mask == 1 {
            self.pntr.bit_mask = 128;
            self.pntr.byte_pntr += 1;
            self.bytes.push(0u8);
        } else {
            self.pntr.bit_mask >>= 1;
        }
    }

    fn increment_pointer(&mut self) {
        if self.pntr.bit_mask == 1 {
            self.pntr.bit_mask = 128;
            self.pntr.byte_pntr += 1;
        } else {
            self.pntr.bit_mask >>= 1;
        }
    }

    pub fn reset_pointer(&mut self) {
        self.pntr = BSPointer::from(0, 0b10000000)
    }
}

impl Iterator for BinaryStack {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.increment_pointer();
        self.read()
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
            self.pntr.byte_pntr, self.pntr.bit_mask, byte_string
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
