mod cpu;
mod opcodes;

fn main() {
    let mut x: u16 = 100;
    let y: u8 = 255;
    // let y: i8 = -1;
    x = x.wrapping_add(y as i8 as u16);
    println!("{}", x);
}
