use crate::cpu::CPU;

mod cpu;
mod opcodes;

fn main() {
    let mut cpu = CPU::new([
        0x69, 255,
        0x69, 10,
        0x85, 0xA,
    ].to_vec());
    cpu.execute_next_instruction();
    cpu.execute_next_instruction();
    cpu.execute_next_instruction();
    dbg!(cpu);
}
