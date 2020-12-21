use crate::cartridge::Cartridge;
use crate::cpu::CPU;

mod cartridge;
mod cpu;
mod disassembler;
mod flags;
mod mapper;
mod opcodes;
mod tests;
mod util;

fn main() {
    let cartridge = Cartridge::from_file("misc/nestest.nes");
    let mut cpu = CPU::new(cartridge);
    cpu.pc = 0xc000;

    loop {
        cpu.clock();
    }
}
