use std::fmt::Debug;

use crate::util::Units;

pub trait Mapper: Debug {
    fn address(&self, address: u16) -> u16;
}

#[derive(Debug)]
pub struct Mapper0 {
    prg_rom_size: usize,
}

impl Mapper0 {
    pub fn new(prg_rom_size: usize) -> Self {
        Self { prg_rom_size }
    }
}

impl Mapper for Mapper0 {
    fn address(&self, address: u16) -> u16 {
        assert!(address > 0x4020);
        if address < 0x8000 {
            unimplemented!("What is this address ({:x}) ??", address);
        }

        let address = address - 0x8000;

        if self.prg_rom_size == 16.KiB() {
            address % 16.KiB()
        } else {
            address
        }
    }
}
