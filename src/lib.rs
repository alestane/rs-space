#![feature(generic_arg_infer)]

extern crate lemurs;

use std::num::Wrapping; 
#[allow(non_camel_case_types)]
mod bits {
    pub type u8 = std::num::Wrapping<core::primitive::u8>;
    pub type u16 = std::num::Wrapping<core::primitive::u16>;
}

struct Shifter {
    data: [u8; 2],
    offset: u8,
}

impl Shifter {
    fn window(&self) -> u8 {
        (u16::from_le_bytes(self.data) << self.offset).to_le_bytes()[1]
    }
    fn align(&mut self, left: u8) {
        self.offset = left & 0x07;
    }
    fn insert(&mut self, bits: u8) {
        self.data = [self.data[1], bits];
    }
}

pub struct Invaders {
    ram: [u8; 0x10000], 
    controls: u8,
    bits: Shifter,
}

impl lemurs::Harness for Invaders {
    fn read(&self, from: bits::u16) -> bits::u8 {
        Wrapping(self.ram[from.0 as usize])
    }
    fn write(&mut self, value: bits::u8, to: bits::u16) {
        self.ram[to.0 as usize] = value.0;
    }
    fn input(&mut self, port: u8) -> bits::u8 {
        match port {
            1 => Wrapping(self.controls),
            3 => Wrapping(self.bits.window()),
            _ => Wrapping(0),
        }
    }
    fn output(&mut self, port: u8, value: bits::u8) {
        match port {
            2 => self.bits.align(value.0), 
            4 => self.bits.insert(value.0),
            _ => ()
        }
    }
}

impl Invaders {
    pub fn install(&mut self) -> lemurs::Machine<Self, &mut Self> {
        lemurs::Machine::new(self)
    }

    pub fn new() -> Self {
        let mut new = Invaders { 
            ram: [0;_],
            controls: 0x00,
            bits: Shifter { data: [0;2], offset: 0 },
        };
        let len = new.ram.len().min(INVADERS.len());
        new.ram[..len].copy_from_slice(&INVADERS[..len]);
        new
    }

    pub fn raster(&self) -> &[u8] {
        &self.ram[0x2400..0x4000]
    }
}

const INVADERS: &[u8] = include_bytes!("invaders.bin");