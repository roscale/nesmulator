use crate::cpu::CPU;

mod cpu;
mod opcodes;
mod tests;
mod flags;
mod util;

fn main() {
    let mut cpu = CPU::new(vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA]);
    for i in 0..41 {
        cpu.execute_next_instruction();
    }

    dbg!(&cpu);

    // let a = 255 as u8;
    // println!("{:b}", a);
    // println!("{:b}", (a >> 3) as u8);

    // let mut cpu = CPU::new([0x69, 42].to_vec());
    // cpu.execute_next_instruction();


    // let a: u8 = 2;
    // let arg: u8 = 255;
    // let sum = a.wrapping_add(arg);
    // println!("Overflow: {}", over());
}
