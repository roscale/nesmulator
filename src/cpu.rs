use std::num::Wrapping;

use crate::opcodes::{AddressingMode, OPCODES};

struct CPU {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
    ram: [u8; 0x800],
    cartridge: Vec<u8>,

    instruction_address: u16,
}

impl CPU {
    pub fn new(cartridge: Vec<u8>) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: 0,
            ram: [0; 0x800],
            cartridge,
            instruction_address: 0,
        }
    }

    pub fn execute_next_instruction(&mut self) {
        let op = self.cartridge[self.pc as usize];
        self.pc += 1;
        let &(
            instruction,
            addressing_mode,
            cycles
        ) = &OPCODES[op as usize];
    }

    /// Returns 1 if the addressing needs 1 more cycle (pages crossed)
    pub fn compute_instruction_address(&mut self, addressing_mode: AddressingMode) -> u8 {
        match addressing_mode {
            AddressingMode::Implicit => 0,
            AddressingMode::Accumulator => 0,
            AddressingMode::Immediate => {
                self.instruction_address = self.cartridge[self.pc as usize] as u16;
                self.pc += 1;
                0
            },
            AddressingMode::ZeroPage => {
                self.instruction_address = self.cartridge[self.pc as usize] as u16;
                self.pc += 1;
                0
            },
            AddressingMode::ZeroPageIndexedX => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = Wrapping(addr) + Wrapping(self.x);
                self.instruction_address = addr.0 as u16;
                0
            },
            AddressingMode::ZeroPageIndexedY => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = Wrapping(addr) + Wrapping(self.y);
                self.instruction_address = addr.0 as u16;
                0
            },
            AddressingMode::Relative => {
                let offset = self.cartridge[self.pc as usize];
                self.pc += 1;
                // We must first transform the offset to i8, then to u16
                let new_pc = self.pc.wrapping_add(offset as i8 as u16);
                self.instruction_address = new_pc;

                if page_of(self.pc) != page_of(new_pc) {
                    1
                } else {
                    0
                }
            },
            AddressingMode::Absolute => {
                let lsb = self.cartridge[self.pc as usize];
                self.pc += 1;
                let msb = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = ((msb as u16) << 8) | (lsb as u16);
                self.instruction_address = addr;
                0
            },
            AddressingMode::AbsoluteIndexedX => {
                let lsb = self.cartridge[self.pc as usize];
                self.pc += 1;
                let msb = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = ((msb as u16) << 8) | (lsb as u16);
                let old_addr = addr;
                let addr = Wrapping(addr) + Wrapping(self.x as u16);
                self.instruction_address = addr.0;
                if page_of(addr.0) != page_of(old_addr) {
                    1
                } else {
                    0
                }
            },
            AddressingMode::AbsoluteIndexedY => {
                let lsb = self.cartridge[self.pc as usize];
                self.pc += 1;
                let msb = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = ((msb as u16) << 8) | (lsb as u16);
                let old_addr = addr;
                let addr = Wrapping(addr) + Wrapping(self.y as u16);
                self.instruction_address = addr.0;
                if page_of(addr.0) != page_of(old_addr) {
                    1
                } else {
                    0
                }
            },
            AddressingMode::Indirect => {
                let lsb = self.cartridge[self.pc as usize];
                self.pc += 1;
                let msb = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = ((msb as u16) << 8) | (lsb as u16);

                let lsb = self.read(addr);
                let msb = self.read(addr + 1);
                let addr = ((msb as u16) << 8) | (lsb as u16);
                self.instruction_address = addr;
                0
            },
            AddressingMode::IndexedIndirect => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let lsb = self.read((Wrapping(addr) + Wrapping(self.x)).0 as u16);
                let msb = self.read((Wrapping(addr) + Wrapping(self.x) + Wrapping(1)).0 as u16);
                let addr = ((msb as u16) << 8) | (lsb as u16);
                self.instruction_address = addr;
                0
            },
            AddressingMode::IndirectIndexed => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let old_addr = addr;
                let lsb = self.read(addr as u16);
                let msb = self.read((Wrapping(addr) + Wrapping(1)).0 as u16);
                let addr = ((msb as u16) << 8) | (lsb as u16);

                let addr = Wrapping(addr) + Wrapping(self.y as u16);
                self.instruction_address = addr.0;
                if page_of(addr.0) != page_of(old_addr as u16) {
                    1
                } else {
                    0
                }
            },
        }
    }

    pub fn get_reference_to(&mut self, address: u16) -> &mut u8 {
        match address {
            0x0000..=0x1FFF => &mut self.ram[address as usize & 0x07FF],
            0x2000..=0x3FFF => unimplemented!("PPU registers"),
            0x4000..=0x4017 => unimplemented!("APU and I/O registers"),
            0x4018..=0x401F => unimplemented!("APU and I/O functionality that is normally disabled"),
            0x4020..=0xFFFF => unimplemented!("Cartridge space"),
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        *self.get_reference_to(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        *self.get_reference_to(address) = value;
    }
}

#[inline]
pub fn page_of(address: u16) -> u8 {
    (address >> 8) as u8
}
