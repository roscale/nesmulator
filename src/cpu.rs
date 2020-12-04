use std::num::Wrapping;

use crate::opcodes::{AddressingMode, Instruction, OPCODES};

#[derive(Debug)]
pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
    ram: [u8; 0x800],
    cartridge: Vec<u8>,

    instruction_target: u16,
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
            instruction_target: 0,
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

        self.compute_instruction_target(addressing_mode);
        self.execute_instruction(instruction, addressing_mode);
    }

    /// Returns 1 if the addressing needs 1 more cycle (pages crossed)
    pub fn compute_instruction_target(&mut self, addressing_mode: AddressingMode) -> u8 {
        match addressing_mode {
            AddressingMode::Implicit => 0,
            AddressingMode::Accumulator => 0,
            AddressingMode::Immediate => {
                self.instruction_target = self.cartridge[self.pc as usize] as u16;
                self.pc += 1;
                0
            },
            AddressingMode::ZeroPage => {
                self.instruction_target = self.cartridge[self.pc as usize] as u16;
                self.pc += 1;
                0
            },
            AddressingMode::ZeroPageIndexedX => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = Wrapping(addr) + Wrapping(self.x);
                self.instruction_target = addr.0 as u16;
                0
            },
            AddressingMode::ZeroPageIndexedY => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = Wrapping(addr) + Wrapping(self.y);
                self.instruction_target = addr.0 as u16;
                0
            },
            AddressingMode::Relative => {
                let offset = self.cartridge[self.pc as usize];
                self.pc += 1;
                // We must cast it to i8 first because the negative sign bit
                // must be repeated when we make it 16 bit.
                let new_pc = self.pc.wrapping_add(offset as i8 as u16);
                self.instruction_target = new_pc;

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
                self.instruction_target = addr;
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
                self.instruction_target = addr.0;
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
                self.instruction_target = addr.0;
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
                self.instruction_target = addr;
                0
            },
            AddressingMode::IndexedIndirect => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let lsb = self.read((Wrapping(addr) + Wrapping(self.x)).0 as u16);
                let msb = self.read((Wrapping(addr) + Wrapping(self.x) + Wrapping(1)).0 as u16);
                let addr = ((msb as u16) << 8) | (lsb as u16);
                self.instruction_target = addr;
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
                self.instruction_target = addr.0;
                if page_of(addr.0) != page_of(old_addr as u16) {
                    1
                } else {
                    0
                }
            },
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction, addressing_mode: AddressingMode) -> bool {
        match instruction {
            Instruction::ADC => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                let carry = self.get_carry_flag() as u8;
                let will_overflow = self.a.checked_add(value)
                    .and_then(|x| x.checked_add(carry))
                    .is_none();
                self.a = (Wrapping(self.a) + Wrapping(value) + Wrapping(carry)).0;
                self.set_carry_flag(will_overflow);
                self.modify_zero_flag(self.a);
                self.modify_negative_flag(self.a);
                true
            },
            Instruction::AND => false,
            Instruction::ASL => false,
            Instruction::BCC => false,
            Instruction::BCS => false,
            Instruction::BEQ => false,
            Instruction::BIT => false,
            Instruction::BMI => false,
            Instruction::BNE => false,
            Instruction::BPL => false,
            Instruction::BRK => false,
            Instruction::BVC => false,
            Instruction::BVS => false,
            Instruction::CLC => false,
            Instruction::CLD => false,
            Instruction::CLI => false,
            Instruction::CLV => false,
            Instruction::CMP => false,
            Instruction::CPX => false,
            Instruction::CPY => false,
            Instruction::DEC => false,
            Instruction::DEX => false,
            Instruction::DEY => false,
            Instruction::EOR => false,
            Instruction::INC => false,
            Instruction::INX => false,
            Instruction::INY => false,
            Instruction::JMP => false,
            Instruction::JSR => false,
            Instruction::LDA => {
                let value = self.read(self.instruction_target);
                self.a = value;
                self.modify_zero_flag(value);
                self.modify_negative_flag(value);
                true
            },
            Instruction::LDX => {
                let value = self.read(self.instruction_target);
                self.x = value;
                self.modify_zero_flag(value);
                self.modify_negative_flag(value);
                true
            },
            Instruction::LDY => {
                let value = self.read(self.instruction_target);
                self.y = value;
                self.modify_zero_flag(value);
                self.modify_negative_flag(value);
                true
            },
            Instruction::LSR => false,
            Instruction::NOP => false,
            Instruction::ORA => false,
            Instruction::PHA => false,
            Instruction::PHP => false,
            Instruction::PLA => false,
            Instruction::PLP => false,
            Instruction::ROL => false,
            Instruction::ROR => false,
            Instruction::RTI => false,
            Instruction::RTS => false,
            Instruction::SBC => false,
            Instruction::SEC => false,
            Instruction::SED => false,
            Instruction::SEI => false,
            Instruction::STA => {
                self.write(self.instruction_target, self.a);
                false
            },
            Instruction::STX => {
                self.write(self.instruction_target, self.x);
                false
            },
            Instruction::STY => {
                self.write(self.instruction_target, self.y);
                false
            },
            Instruction::TAX => false,
            Instruction::TAY => false,
            Instruction::TSX => false,
            Instruction::TXA => false,
            Instruction::TXS => false,
            Instruction::TYA => false,
        }
    }

    #[inline]
    pub fn read(&mut self, address: u16) -> u8 {
        *self.get_reference_to(address)
    }

    #[inline]
    pub fn write(&mut self, address: u16, value: u8) {
        *self.get_reference_to(address) = value;
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

    #[inline]
    pub fn modify_zero_flag(&mut self, value: u8) {
        self.set_flag(1 << 1, value == 0);
    }

    #[inline]
    pub fn modify_negative_flag(&mut self, value: u8) {
        self.set_flag(1 << 7, value & 0b1000_0000 != 0);
    }

    #[inline]
    pub fn get_carry_flag(&mut self) -> bool {
        self.p & 0b0000_0001 != 0
    }

    #[inline]
    pub fn set_carry_flag(&mut self, set: bool) {
        self.set_flag(1 << 0, set);
    }

    pub fn set_flag(&mut self, mask: u8, set: bool) {
        let mut flags = self.p & !mask; // clear the flag
        if set {
            flags |= mask;
        };
        self.p = flags;
    }
}

#[inline]
pub fn page_of(address: u16) -> u8 {
    (address >> 8) as u8
}
