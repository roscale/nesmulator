use crate::opcodes::AddressingMode::*;
use crate::opcodes::Instruction::*;

#[rustfmt::skip]
pub const OPCODES: [(Instruction, AddressingMode, u8); 256] = [
    // 00
    (BRK, Immediate, 7), (ORA, IndexedIndirect, 6), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (NOP, Implicit, 3), (ORA, ZeroPage, 3), (ASL, ZeroPage, 5), (NOP, Implicit, 5),
    (PHP, Implicit, 3), (ORA, Immediate, 2), (ASL, Accumulator, 2), (NOP, Implicit, 2),
    (NOP, Implicit, 4), (ORA, Absolute, 4), (ASL, Absolute, 6), (NOP, Implicit, 6),
    // 10
    (BPL, Relative, 2), (ORA, IndirectIndexed, 5), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (NOP, Implicit, 4), (ORA, ZeroPageIndexedX, 4), (ASL, ZeroPageIndexedX, 6), (NOP, Implicit, 6),
    (CLC, Implicit, 2), (ORA, AbsoluteIndexedY, 4), (NOP, Implicit, 2), (NOP, Implicit, 7),
    (NOP, Implicit, 4), (ORA, AbsoluteIndexedX, 4), (ASL, AbsoluteIndexedX, 7), (NOP, Implicit, 7),
    // 20
    (JSR, Absolute, 6), (AND, IndexedIndirect, 6), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (BIT, ZeroPage, 3), (AND, ZeroPage, 3), (ROL, ZeroPage, 5), (NOP, Implicit, 5),
    (PLP, Implicit, 4), (AND, Immediate, 2), (ROL, Accumulator, 2), (NOP, Implicit, 2),
    (BIT, Absolute, 4), (AND, Absolute, 4), (ROL, Absolute, 6), (NOP, Implicit, 6),
    // 30
    (BMI, Relative, 2), (AND, IndirectIndexed, 5), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (NOP, Implicit, 4), (AND, ZeroPageIndexedX, 4), (ROL, ZeroPageIndexedX, 6), (NOP, Implicit, 6),
    (SEC, Implicit, 2), (AND, AbsoluteIndexedY, 4), (NOP, Implicit, 2), (NOP, Implicit, 7),
    (NOP, Implicit, 4), (AND, AbsoluteIndexedX, 4), (ROL, AbsoluteIndexedX, 7), (NOP, Implicit, 7),
    // 40
    (RTI, Implicit, 6), (EOR, IndexedIndirect, 6), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (NOP, Implicit, 3), (EOR, ZeroPage, 3), (LSR, ZeroPage, 5), (NOP, Implicit, 5),
    (PHA, Implicit, 3), (EOR, Immediate, 2), (LSR, Accumulator, 2), (NOP, Implicit, 2),
    (JMP, Absolute, 3), (EOR, Absolute, 4), (LSR, Absolute, 6), (NOP, Implicit, 6),
    // 50
    (BVC, Relative, 2), (EOR, IndirectIndexed, 5), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (NOP, Implicit, 4), (EOR, ZeroPageIndexedX, 4), (LSR, ZeroPageIndexedX, 6), (NOP, Implicit, 6),
    (CLI, Implicit, 2), (EOR, AbsoluteIndexedY, 4), (NOP, Implicit, 2), (NOP, Implicit, 7),
    (NOP, Implicit, 4), (EOR, AbsoluteIndexedX, 4), (LSR, AbsoluteIndexedX, 7), (NOP, Implicit, 7),
    // 60
    (RTS, Implicit, 6), (ADC, IndexedIndirect, 6), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (NOP, Implicit, 3), (ADC, ZeroPage, 3), (ROR, ZeroPage, 5), (NOP, Implicit, 5),
    (PLA, Implicit, 4), (ADC, Immediate, 2), (ROR, Accumulator, 2), (NOP, Implicit, 2),
    (JMP, Indirect, 5), (ADC, Absolute, 4), (ROR, Absolute, 6), (NOP, Implicit, 6),
    // 70
    (BVS, Relative, 2), (ADC, IndirectIndexed, 5), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (NOP, Implicit, 4), (ADC, ZeroPageIndexedX, 4), (ROR, ZeroPageIndexedX, 6), (NOP, Implicit, 6),
    (SEI, Implicit, 2), (ADC, AbsoluteIndexedY, 4), (NOP, Implicit, 2), (NOP, Implicit, 7),
    (NOP, Implicit, 4), (ADC, AbsoluteIndexedX, 4), (ROR, AbsoluteIndexedX, 7), (NOP, Implicit, 7),
    // 80
    (NOP, Implicit, 2), (STA, IndexedIndirect, 6), (NOP, Implicit, 2), (NOP, Implicit, 6),
    (STY, ZeroPage, 3), (STA, ZeroPage, 3), (STX, ZeroPage, 3), (NOP, Implicit, 3),
    (DEY, Implicit, 2), (NOP, Implicit, 2), (TXA, Implicit, 2), (NOP, Implicit, 2),
    (STY, Absolute, 4), (STA, Absolute, 4), (STX, Absolute, 4), (NOP, Implicit, 4),
    // 90
    (BCC, Relative, 2), (STA, IndirectIndexed, 6), (NOP, Implicit, 2), (NOP, Implicit, 6),
    (STY, ZeroPageIndexedX, 4), (STA, ZeroPageIndexedX, 4), (STX, ZeroPageIndexedY, 4), (NOP, Implicit, 4),
    (TYA, Implicit, 2), (STA, AbsoluteIndexedY, 5), (TXS, Implicit, 2), (NOP, Implicit, 5),
    (NOP, Implicit, 5), (STA, AbsoluteIndexedX, 5), (NOP, Implicit, 5), (NOP, Implicit, 5),
    // A0
    (LDY, Immediate, 2), (LDA, IndexedIndirect, 6), (LDX, Immediate, 2), (NOP, Implicit, 6),
    (LDY, ZeroPage, 3), (LDA, ZeroPage, 3), (LDX, ZeroPage, 3), (NOP, Implicit, 3),
    (TAY, Implicit, 2), (LDA, Immediate, 2), (TAX, Implicit, 2), (NOP, Implicit, 2),
    (LDY, Absolute, 4), (LDA, Absolute, 4), (LDX, Absolute, 4), (NOP, Implicit, 4),
    // B0
    (BCS, Relative, 2), (LDA, IndirectIndexed, 5), (NOP, Implicit, 2), (NOP, Implicit, 5),
    (LDY, ZeroPageIndexedX, 4), (LDA, ZeroPageIndexedX, 4), (LDX, ZeroPageIndexedY, 4), (NOP, Implicit, 4),
    (CLV, Implicit, 2), (LDA, AbsoluteIndexedY, 4), (TSX, Implicit, 2), (NOP, Implicit, 4),
    (LDY, AbsoluteIndexedX, 4), (LDA, AbsoluteIndexedX, 4), (LDX, AbsoluteIndexedY, 4), (NOP, Implicit, 4),
    // C0
    (CPY, Immediate, 2), (CMP, IndexedIndirect, 6), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (CPY, ZeroPage, 3), (CMP, ZeroPage, 3), (DEC, ZeroPage, 5), (NOP, Implicit, 5),
    (INY, Implicit, 2), (CMP, Immediate, 2), (DEX, Implicit, 2), (NOP, Implicit, 2),
    (CPY, Absolute, 4), (CMP, Absolute, 4), (DEC, Absolute, 6), (NOP, Implicit, 6),
    // D0
    (BNE, Relative, 2), (CMP, IndirectIndexed, 5), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (NOP, Implicit, 4), (CMP, ZeroPageIndexedX, 4), (DEC, ZeroPageIndexedX, 6), (NOP, Implicit, 6),
    (CLD, Implicit, 2), (CMP, AbsoluteIndexedY, 4), (NOP, Implicit, 2), (NOP, Implicit, 7),
    (NOP, Implicit, 4), (CMP, AbsoluteIndexedX, 4), (DEC, AbsoluteIndexedX, 7), (NOP, Implicit, 7),
    // E0
    (CPX, Immediate, 2), (SBC, IndexedIndirect, 6), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (CPX, ZeroPage, 3), (SBC, ZeroPage, 3), (INC, ZeroPage, 5), (NOP, Implicit, 5),
    (INX, Implicit, 2), (SBC, Immediate, 2), (NOP, Implicit, 2), (SBC, Implicit, 2),
    (CPX, Absolute, 4), (SBC, Absolute, 4), (INC, Absolute, 6), (NOP, Implicit, 6),
    // F0
    (BEQ, Relative, 2), (SBC, IndirectIndexed, 5), (NOP, Implicit, 2), (NOP, Implicit, 8),
    (NOP, Implicit, 4), (SBC, ZeroPageIndexedX, 4), (INC, ZeroPageIndexedX, 6), (NOP, Implicit, 6),
    (SED, Implicit, 2), (SBC, AbsoluteIndexedY, 4), (NOP, Implicit, 2), (NOP, Implicit, 7),
    (NOP, Implicit, 4), (SBC, AbsoluteIndexedX, 4), (INC, AbsoluteIndexedX, 7), (NOP, Implicit, 7),
];

#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub enum Instruction {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC,
    CLD, CLI, CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, 
    JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA, PLP, ROL, ROR, RTI, 
    RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,
}

#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    Relative,
    Absolute,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}
