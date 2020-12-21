use crate::cpu::CPU;
use crate::opcodes::{AddressingMode, Instruction, OPCODES};

impl CPU {
    pub fn disassemble_current_instruction(&mut self) {
        let op = self.read(self.pc);
        let (instruction, addressing_mode, _) = OPCODES[op as usize];

        print!("{:04X}  ", self.pc);

        let bytes = match addressing_mode {
            AddressingMode::Implicit => 1,
            AddressingMode::Accumulator => 1,
            AddressingMode::Immediate => 2,
            AddressingMode::ZeroPage => 2,
            AddressingMode::ZeroPageIndexedX => 2,
            AddressingMode::ZeroPageIndexedY => 2,
            AddressingMode::Relative => 2,
            AddressingMode::Absolute => 3,
            AddressingMode::AbsoluteIndexedX => 3,
            AddressingMode::AbsoluteIndexedY => 3,
            AddressingMode::Indirect => 3,
            AddressingMode::IndexedIndirect => 2,
            AddressingMode::IndirectIndexed => 2,
        };

        let mut bytes_str = String::new();
        for i in 0..bytes {
            bytes_str += &format!("{:02X} ", self.read(self.pc + i));
        }
        print!("{:<10}", bytes_str);

        print!("{:?} ", instruction);

        let pc = self.pc + 1;
        let arg = match addressing_mode {
            AddressingMode::Implicit => String::new(),
            AddressingMode::Accumulator => "A".to_string(),
            AddressingMode::Immediate => format!("#${:02X}", self.read(pc)),
            AddressingMode::ZeroPage => {
                let addr = self.read(pc);
                format!("${:02X} = {:02X}", addr, self.read(addr as u16))
            }
            AddressingMode::ZeroPageIndexedX => {
                let addr = self.read(pc);
                let addr_plus_x = addr.wrapping_add(self.x);
                format!(
                    "${:02X},X @ {:02X} = {:02X}",
                    addr,
                    addr_plus_x,
                    self.read(addr_plus_x as u16)
                )
            }
            AddressingMode::ZeroPageIndexedY => {
                let addr = self.read(pc);
                let addr_plus_y = addr.wrapping_add(self.y);
                format!(
                    "${:02X},Y @ {:02X} = {:02X}",
                    addr,
                    addr_plus_y,
                    self.read(addr_plus_y as u16)
                )
            }
            AddressingMode::Relative => format!("${:04X}", pc + 1 + self.read(pc) as i8 as u16),
            AddressingMode::Absolute => {
                let is_jump_instruction = match instruction {
                    Instruction::JSR => true,
                    Instruction::JMP => true,
                    _ => false,
                };
                if !is_jump_instruction {
                    let addr = self.read_u16(pc);
                    format!("${:04X} = {:02X}", self.read_u16(pc), self.read(addr))
                } else {
                    format!("${:04X}", self.read_u16(pc))
                }
            }
            AddressingMode::AbsoluteIndexedX => {
                let addr = {
                    let lsb = self.read(pc);
                    let msb = self.read(pc + 1);
                    u16::from_le_bytes([lsb, msb])
                };
                let addr_plus_x = addr.wrapping_add(self.x as u16);
                format!(
                    "${:04X},X @ {:04X} = {:02X}",
                    addr,
                    addr_plus_x,
                    self.read(addr_plus_x)
                )
            }
            AddressingMode::AbsoluteIndexedY => {
                let addr = {
                    let lsb = self.read(pc);
                    let msb = self.read(pc + 1);
                    u16::from_le_bytes([lsb, msb])
                };
                let addr_plus_y = addr.wrapping_add(self.y as u16);
                format!(
                    "${:04X},Y @ {:04X} = {:02X}",
                    addr,
                    addr_plus_y,
                    self.read(addr_plus_y)
                )
            }
            AddressingMode::Indirect => {
                let (addr, lsb) = {
                    let lsb = self.read(pc);
                    let msb = self.read(pc + 1);
                    (u16::from_le_bytes([lsb, msb]), lsb)
                };
                let indirect_addr = {
                    // Hardware bug
                    if lsb == 0xFF {
                        let lsb = self.read(addr);
                        let msb = self.read(addr & 0xFF00);
                        u16::from_le_bytes([lsb, msb])
                    } else {
                        let lsb = self.read(addr);
                        let msb = self.read(addr + 1);
                        u16::from_le_bytes([lsb, msb])
                    }
                };
                format!("(${:04X}) = {:04X}", addr, indirect_addr)
            }
            AddressingMode::IndexedIndirect => {
                let arg = self.read(pc);
                let arg_plus_x = arg.wrapping_add(self.x);
                let addr = {
                    let lsb = self.read(arg.wrapping_add(self.x) as u16);
                    let msb = self.read(arg.wrapping_add(self.x).wrapping_add(1) as u16);
                    u16::from_le_bytes([lsb, msb])
                };
                format!(
                    "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    arg,
                    arg_plus_x,
                    addr,
                    self.read(addr)
                )
            }
            AddressingMode::IndirectIndexed => {
                let arg = self.read(pc);
                let addr = {
                    let lsb = self.read(arg as u16);
                    let msb = self.read(arg.wrapping_add(1) as u16);
                    u16::from_le_bytes([lsb, msb])
                };
                let addr_plus_y = addr.wrapping_add(self.y as u16);
                format!(
                    "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    arg,
                    addr,
                    addr_plus_y,
                    self.read(addr_plus_y)
                )
            }
        };

        print!("{:<28}", arg);

        print!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.a,
            self.x,
            self.y,
            self.flags.to_byte(),
            self.s
        );

        println!();
    }

    pub fn read_u16(&mut self, address: u16) -> u16 {
        let lsb = self.read(address);
        let msb = self.read(address + 1);
        u16::from_le_bytes([lsb, msb])
    }
}
