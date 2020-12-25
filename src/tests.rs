#[cfg(test)]
mod cpu {
    use std::fs::File;
    use std::io::Read;

    use crate::cartridge::Cartridge;
    use crate::cpu::CPU;
    use crate::util::BitOperations;

    #[test]
    fn nestest() {
        let cartridge = Cartridge::from_file("misc/nestest.nes");
        let mut cpu = CPU::new(cartridge);
        cpu.enable_logging(true);
        cpu.pc = 0xc000;

        // Starting from 0xC6BD, nestest tests illegal instructions that we don't implement
        while cpu.pc != 0xC6BD {
            cpu.clock();
        }
        let my_logs = cpu.logs;

        let mut nestest_logs_file = File::open("misc/nestest.log").unwrap();
        let mut nestest_logs = String::new();
        nestest_logs_file.read_to_string(&mut nestest_logs).unwrap();

        assert_eq!(my_logs, nestest_logs);
    }

    #[test]
    fn bit_operations() {
        let mut result = 0u16;
        let a = 0b0101_1111;
        let b = 0b0000_1010;
        let mut c = 0b1100_0000;
        c.set_bits_all(2..=3, true);
        result.set_bits(0..=3, a.get_bits(4..=7));
        result.set_bits(4..=7, b.get_bits(0..=3));
        result.set_bits(8..=15, c.get_bits(0..=7));
        assert_eq!(result, 0b1100_1100_1010_0101);
    }
}
