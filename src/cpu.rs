use crate::flags::CPUFlags;
use crate::opcodes::{AddressingMode, Instruction, OPCODES};
use crate::util::{BitOperations, page_of};

#[derive(Debug)]
pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub flags: CPUFlags,
    pub ram: [u8; 0x800],
    pub cartridge: Vec<u8>,

    pub instruction_target: u16,
    pub cycles_remaining: u8,
}

impl CPU {
    pub fn new(cartridge: Vec<u8>) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0xFF,
            flags: CPUFlags::new(),
            ram: [0; 0x800],
            cartridge,
            instruction_target: 0,
            cycles_remaining: 0,
        }
    }

    pub fn run(&mut self) {
        if self.cycles_remaining > 0 {
            self.cycles_remaining -= 1;
            return;
        }
        self.execute_next_instruction();
        self.cycles_remaining -= 1;
    }

    pub fn execute_next_instruction(&mut self) {
        let op = self.cartridge[self.pc as usize];
        self.pc += 1;
        let (
            instruction,
            addressing_mode,
            cycles
        ) = OPCODES[op as usize];

        self.cycles_remaining = cycles;
        let additional_cycles = self.compute_instruction_target(addressing_mode);
        let can_take_more_cycles = self.execute_instruction(instruction, addressing_mode);

        // Some instructions don't take more cycles when a new page is crossed.
        if can_take_more_cycles {
            self.cycles_remaining += additional_cycles;
        }
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
                let addr = addr.wrapping_add(self.x);
                self.instruction_target = addr as u16;
                0
            },
            AddressingMode::ZeroPageIndexedY => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = addr.wrapping_add(self.y);
                self.instruction_target = addr as u16;
                0
            },
            // This addressing mode is only used for branching instructions.
            AddressingMode::Relative => {
                let offset = self.cartridge[self.pc as usize];
                self.pc += 1;
                // We must cast it to i8 first because the negative sign bit
                // must be repeated when we make it 16 bit.
                let absolute_offset = self.pc.wrapping_add(offset as i8 as u16);
                self.instruction_target = absolute_offset;
                if page_of(self.pc) != page_of(absolute_offset) {
                    2 // Special case
                } else {
                    0
                }
            },
            AddressingMode::Absolute => {
                let addr = {
                    let lsb = self.cartridge[self.pc as usize];
                    self.pc += 1;
                    let msb = self.cartridge[self.pc as usize];
                    self.pc += 1;
                    u16::from_le_bytes([lsb, msb])
                };
                self.instruction_target = addr;
                0
            },
            AddressingMode::AbsoluteIndexedX => {
                let addr = {
                    let lsb = self.cartridge[self.pc as usize];
                    self.pc += 1;
                    let msb = self.cartridge[self.pc as usize];
                    self.pc += 1;
                    u16::from_le_bytes([lsb, msb])
                };
                let old_addr = addr;
                let addr = addr.wrapping_add(self.x as u16);
                self.instruction_target = addr;
                if page_of(addr) != page_of(old_addr) {
                    1
                } else {
                    0
                }
            },
            AddressingMode::AbsoluteIndexedY => {
                let addr = {
                    let lsb = self.cartridge[self.pc as usize];
                    self.pc += 1;
                    let msb = self.cartridge[self.pc as usize];
                    self.pc += 1;
                    u16::from_le_bytes([lsb, msb])
                };
                let old_addr = addr;
                let addr = addr.wrapping_add(self.y as u16);
                self.instruction_target = addr;
                if page_of(addr) != page_of(old_addr) {
                    1
                } else {
                    0
                }
            },
            AddressingMode::Indirect => {
                let addr = {
                    let lsb = self.cartridge[self.pc as usize];
                    self.pc += 1;
                    let msb = self.cartridge[self.pc as usize];
                    self.pc += 1;
                    u16::from_le_bytes([lsb, msb])
                };
                let addr = {
                    let lsb = self.read(addr);
                    let msb = self.read(addr + 1);
                    u16::from_le_bytes([lsb, msb])
                };
                self.instruction_target = addr;
                0
            },
            AddressingMode::IndexedIndirect => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let addr = {
                    let lsb = self.read(addr.wrapping_add(self.x) as u16);
                    let msb = self.read(addr.wrapping_add(self.x).wrapping_add(1) as u16);
                    u16::from_le_bytes([lsb, msb])
                };
                self.instruction_target = addr;
                0
            },
            AddressingMode::IndirectIndexed => {
                let addr = self.cartridge[self.pc as usize];
                self.pc += 1;
                let old_addr = addr;
                let addr = {
                    let lsb = self.read(addr as u16);
                    let msb = self.read(addr.wrapping_add(1) as u16);
                    u16::from_le_bytes([lsb, msb])
                };
                let addr = addr.wrapping_add(self.y as u16);
                self.instruction_target = addr;
                if page_of(addr) != page_of(old_addr as u16) {
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
                let carry = self.flags.carry as u8;
                let will_carry = self.a.checked_add(value)
                    .and_then(|x| x.checked_add(carry))
                    .is_none();

                let old_a = self.a;
                self.a = self.a.wrapping_add(value).wrapping_add(carry);

                self.flags.carry = will_carry;
                self.flags.overflow = old_a.get_bit(7) == value.get_bit(7)
                    && old_a.get_bit(7) != self.a.get_bit(7);
                self.flags.zero = self.a == 0;
                self.flags.negative = self.a.get_bit(7);
                true
            },
            Instruction::AND => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                self.a &= value;

                self.flags.zero = self.a == 0;
                self.flags.negative = self.a.get_bit(7);
                true
            },
            Instruction::ASL => {
                if let AddressingMode::Accumulator = addressing_mode {
                    self.flags.carry = self.a.get_bit(7);
                    self.a <<= 1;
                    self.flags.zero = self.a == 0;
                    self.flags.negative = self.a.get_bit(7);
                } else {
                    let mut value = self.read(self.instruction_target);
                    self.flags.carry = value.get_bit(7);
                    value <<= 1;
                    self.write(self.instruction_target, value);
                    self.flags.zero = value == 0;
                    self.flags.negative = value.get_bit(7);
                }
                false
            },
            Instruction::BCC => {
                if !self.flags.carry {
                    self.pc = self.instruction_target;
                    self.cycles_remaining += 1;
                }
                true
            },
            Instruction::BCS => {
                if self.flags.carry {
                    self.pc = self.instruction_target;
                    self.cycles_remaining += 1;
                }
                true
            },
            Instruction::BEQ => {
                if self.flags.zero {
                    self.pc = self.instruction_target;
                    self.cycles_remaining += 1;
                }
                true
            },
            Instruction::BIT => {
                let value = self.read(self.instruction_target);
                self.flags.zero = (self.a & value) == 0;
                self.flags.negative = value.get_bit(7);
                self.flags.overflow = value.get_bit(6);
                false
            },
            Instruction::BMI => {
                if self.flags.negative {
                    self.pc = self.instruction_target;
                    self.cycles_remaining += 1;
                }
                true
            },
            Instruction::BNE => {
                if !self.flags.zero {
                    self.pc = self.instruction_target;
                    self.cycles_remaining += 1;
                }
                true
            },
            Instruction::BPL => {
                if !self.flags.negative {
                    self.pc = self.instruction_target;
                    self.cycles_remaining += 1;
                }
                true
            },
            Instruction::BRK => {
                self.push_u16(self.pc);
                self.push_u8(self.flags.to_byte());

                // Load the IRQ interrupt
                self.pc = {
                    let lsb = self.read(0xFFFE);
                    let msb = self.read(0xFFFF);
                    u16::from_le_bytes([lsb, msb])
                };

                self.flags.break_command = 1;
                false
            },
            Instruction::BVC => {
                if !self.flags.overflow {
                    self.pc = self.instruction_target;
                    self.cycles_remaining += 1;
                }
                true
            },
            Instruction::BVS => {
                if self.flags.overflow {
                    self.pc = self.instruction_target;
                    self.cycles_remaining += 1;
                }
                true
            },
            Instruction::CLC => {
                self.flags.carry = false;
                false
            },
            Instruction::CLD => {
                self.flags.decimal_mode = false;
                false
            },
            Instruction::CLI => {
                self.flags.interrupt_disable = false;
                false
            },
            Instruction::CLV => {
                self.flags.overflow = false;
                false
            },
            Instruction::CMP => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                self.flags.carry = self.a >= value;
                self.flags.zero = self.a == value;
                self.flags.negative = self.a.wrapping_sub(value).get_bit(7);
                true
            },
            Instruction::CPX => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                self.flags.carry = self.x >= value;
                self.flags.zero = self.x == value;
                self.flags.negative = self.x.wrapping_sub(value).get_bit(7);
                false
            },
            Instruction::CPY => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                self.flags.carry = self.y >= value;
                self.flags.zero = self.y == value;
                self.flags.negative = self.y.wrapping_sub(value).get_bit(7);
                false
            },
            Instruction::DEC => {
                let value = self.read(self.instruction_target);
                let result = value.wrapping_sub(1);
                self.write(self.instruction_target, result);
                self.flags.zero = result == 0;
                self.flags.negative = result.get_bit(7);
                false
            },
            Instruction::DEX => {
                self.x = self.x.wrapping_sub(1);
                self.flags.zero = self.x == 0;
                self.flags.negative = self.x.get_bit(7);
                false
            },
            Instruction::DEY => {
                self.y = self.y.wrapping_sub(1);
                self.flags.zero = self.y == 0;
                self.flags.negative = self.y.get_bit(7);
                false
            },
            Instruction::EOR => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                self.a ^= value;
                self.flags.zero = self.a == 0;
                self.flags.negative = self.a.get_bit(7);
                true
            },
            Instruction::INC => {
                let value = self.read(self.instruction_target);
                let result = value.wrapping_add(1);
                self.write(self.instruction_target, result);
                self.flags.zero = result == 0;
                self.flags.negative = result.get_bit(7);
                false
            },
            Instruction::INX => {
                self.x = self.x.wrapping_add(1);
                self.flags.zero = self.x == 0;
                self.flags.negative = self.x.get_bit(7);
                false
            },
            Instruction::INY => {
                self.y = self.y.wrapping_add(1);
                self.flags.zero = self.y == 0;
                self.flags.negative = self.y.get_bit(7);
                false
            },
            Instruction::JMP => {
                self.pc = self.instruction_target;
                false
            },
            Instruction::JSR => {
                let pc_minus_one = self.pc.wrapping_sub(1);
                self.push_u16(pc_minus_one);
                self.pc = self.instruction_target;
                false
            },
            Instruction::LDA => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                self.a = value;
                self.flags.zero = value == 0;
                self.flags.negative = value.get_bit(7);
                true
            },
            Instruction::LDX => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                self.x = value;
                self.flags.zero = value == 0;
                self.flags.negative = value.get_bit(7);
                true
            },
            Instruction::LDY => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                self.y = value;
                self.flags.zero = value == 0;
                self.flags.negative = value.get_bit(7);
                true
            },
            Instruction::LSR => {
                if let AddressingMode::Accumulator = addressing_mode {
                    self.flags.carry = self.a.get_bit(0);
                    self.a >>= 1;
                    self.flags.zero = self.a == 0;
                    self.flags.negative = self.a.get_bit(7);
                } else {
                    let value = self.read(self.instruction_target);
                    self.flags.carry = value.get_bit(0);
                    let result = value >> 1;
                    self.write(self.instruction_target, result);
                    self.flags.zero = result == 0;
                    self.flags.negative = result.get_bit(7);
                }
                false
            },
            Instruction::NOP => false,
            Instruction::ORA => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                self.a |= value;
                self.flags.zero = self.a == 0;
                self.flags.negative = self.a.get_bit(7);
                true
            },
            Instruction::PHA => {
                self.push_u8(self.a);
                false
            },
            Instruction::PHP => {
                self.push_u8(self.flags.to_byte());
                false
            },
            Instruction::PLA => {
                self.a = self.pop_u8();
                self.flags.zero = self.a == 0;
                self.flags.negative = self.a.get_bit(7);
                false
            },
            Instruction::PLP => {
                self.flags = CPUFlags::from_byte(self.pop_u8());
                false
            },
            Instruction::ROL => {
                if let AddressingMode::Accumulator = addressing_mode {
                    let new_carry = self.a.get_bit(7);
                    self.a <<= 1;
                    self.a.set_bit(0, self.flags.carry);
                    self.flags.carry = new_carry;
                    self.flags.zero = self.a == 0;
                    self.flags.negative = self.a.get_bit(7);
                } else {
                    let mut value = self.read(self.instruction_target);
                    let new_carry = value.get_bit(7);
                    value <<= 1;
                    value.set_bit(0, self.flags.carry);
                    self.write(self.instruction_target, value);
                    self.flags.carry = new_carry;
                    self.flags.zero = value == 0;
                    self.flags.negative = value.get_bit(7);
                }
                false
            },
            Instruction::ROR => {
                if let AddressingMode::Accumulator = addressing_mode {
                    let new_carry = self.a.get_bit(0);
                    self.a >>= 1;
                    self.a.set_bit(7, self.flags.carry);
                    self.flags.carry = new_carry;
                    self.flags.zero = self.a == 0;
                    self.flags.negative = self.a.get_bit(7);
                } else {
                    let mut value = self.read(self.instruction_target);
                    let new_carry = value.get_bit(0);
                    value >>= 1;
                    value.set_bit(7, self.flags.carry);
                    self.write(self.instruction_target, value);
                    self.flags.carry = new_carry;
                    self.flags.zero = value == 0;
                    self.flags.negative = value.get_bit(7);
                }
                false
            },
            Instruction::RTI => {
                self.flags = CPUFlags::from_byte(self.pop_u8());
                self.pc = self.pop_u16();
                false
            },
            Instruction::RTS => {
                self.pc = self.pop_u16() + 1;
                false
            },
            Instruction::SBC => {
                let value = if let AddressingMode::Immediate = addressing_mode {
                    self.instruction_target as u8
                } else {
                    self.read(self.instruction_target)
                };
                let carry = !self.flags.carry as u8;
                let will_carry = self.a.checked_sub(value)
                    .and_then(|x| x.checked_sub(carry))
                    .is_none();

                let old_a = self.a;
                self.a = self.a.wrapping_sub(value).wrapping_sub(carry);

                self.flags.carry = !will_carry;
                self.flags.overflow = old_a.get_bit(7) == value.get_bit(7)
                    && old_a.get_bit(7) != self.a.get_bit(7);
                self.flags.zero = self.a == 0;
                self.flags.negative = self.a.get_bit(7);
                true
            }
            Instruction::SEC => {
                self.flags.carry = true;
                false
            },
            Instruction::SED => {
                self.flags.decimal_mode = true;
                false
            },
            Instruction::SEI => {
                self.flags.interrupt_disable = true;
                false
            },
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
            Instruction::TAX => {
                self.x = self.a;
                self.flags.zero = self.x == 0;
                self.flags.negative = self.x.get_bit(7);
                false
            },
            Instruction::TAY => {
                self.y = self.a;
                self.flags.zero = self.y == 0;
                self.flags.negative = self.y.get_bit(7);
                false
            },
            Instruction::TSX => {
                self.x = self.s;
                self.flags.zero = self.x == 0;
                self.flags.negative = self.x.get_bit(7);
                false
            },
            Instruction::TXA => {
                self.a = self.x;
                self.flags.zero = self.a == 0;
                self.flags.negative = self.a.get_bit(7);
                false
            },
            Instruction::TXS => {
                self.s = self.x;
                false
            },
            Instruction::TYA => {
                self.a = self.y;
                self.flags.zero = self.a == 0;
                self.flags.negative = self.a.get_bit(7);
                false
            },
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

    pub fn push_u8(&mut self, value: u8) {
        self.write(0x100 + self.s as u16, value);
        self.s -= 1;
    }

    pub fn push_u16(&mut self, value: u16) {
        let [lsb, msb] = value.to_le_bytes();
        self.push_u8(lsb);
        self.push_u8(msb);
    }

    pub fn pop_u8(&mut self) -> u8 {
        self.s += 1;
        self.read(0x100 + self.s as u16)
    }

    pub fn pop_u16(&mut self) -> u16 {
        let msb = self.pop_u8();
        let lsb = self.pop_u8();
        u16::from_le_bytes([lsb, msb])
    }
}
