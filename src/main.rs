mod cpu;
mod opcodes;
mod tests;
mod flags;
mod util;

fn main() {
    let a = 255 as u8;
    println!("{:b}", a);
    println!("{:b}", (a >> 3) as u8);

    // let mut cpu = CPU::new([0x69, 42].to_vec());
    // cpu.execute_next_instruction();


    // let a: u8 = 2;
    // let arg: u8 = 255;
    // let sum = a.wrapping_add(arg);
    // println!("Overflow: {}", over());
}
