#[cfg(test)]
mod tests {
    use crate::cpu::CPU;
    use crate::opcodes::AddressingMode;

    //
    // Addressing modes
    //

    #[test]
    fn immediate() {
        let mut cpu = CPU::new([42].to_vec());
        assert_eq!(cpu.compute_instruction_target(AddressingMode::Immediate), 0);
        assert_eq!(cpu.instruction_target, 42);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn zero_page() {
        let mut cpu = CPU::new([42].to_vec());
        assert_eq!(cpu.compute_instruction_target(AddressingMode::ZeroPage), 0);
        assert_eq!(cpu.instruction_target, 42);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn zero_page_indexed_x() {
        let mut cpu = CPU::new([42].to_vec());
        cpu.x = 1;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::ZeroPageIndexedX), 0);
        assert_eq!(cpu.instruction_target, 43);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn zero_page_indexed_y() {
        let mut cpu = CPU::new([42].to_vec());
        cpu.y = 1;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::ZeroPageIndexedY), 0);
        assert_eq!(cpu.instruction_target, 43);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn relative() {
        // Without page crossing
        let mut cpu = CPU::new([2].to_vec());
        assert_eq!(cpu.compute_instruction_target(AddressingMode::Relative), 0);
        assert_eq!(cpu.instruction_target, 3);
        assert_eq!(cpu.pc, 1);

        // With page crossing
        let mut cpu = CPU::new({
            let mut cartridge: [u8; 251] = [0; 251];
            cartridge[250] = 127;
            cartridge.to_vec()
        });
        cpu.pc = 250;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::Relative), 2);
        assert_eq!(cpu.instruction_target, 250 + 127 + 1);
        assert_eq!(cpu.pc, 251);
    }

    #[test]
    fn absolute() {
        let mut cpu = CPU::new([0xCD, 0xAB].to_vec());
        assert_eq!(cpu.compute_instruction_target(AddressingMode::Absolute), 0);
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.instruction_target, 0xABCD);
    }

    #[test]
    fn absolute_indexed_x() {
        // Without page crossing
        let mut cpu = CPU::new([0xCD, 0xAB].to_vec());
        cpu.x = 1;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::AbsoluteIndexedX), 0);
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.instruction_target, 0xABCE);

        // With page crossing
        let mut cpu = CPU::new([0xFF, 0xAB].to_vec());
        cpu.x = 1;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::AbsoluteIndexedX), 1);
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.instruction_target, 0xAC00);
    }

    #[test]
    fn absolute_indexed_y() {
        // Without page crossing
        let mut cpu = CPU::new([0xCD, 0xAB].to_vec());
        cpu.y = 1;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::AbsoluteIndexedY), 0);
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.instruction_target, 0xABCE);

        // With page crossing
        let mut cpu = CPU::new([0xFF, 0xAB].to_vec());
        cpu.y = 1;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::AbsoluteIndexedY), 1);
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.instruction_target, 0xAC00);
    }

    #[test]
    fn indirect() {
        let mut cpu = CPU::new([0xFA, 0x07].to_vec());
        cpu.ram[0x07FA] = 0xCD;
        cpu.ram[0x07FB] = 0xAB;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::Indirect), 0);
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.instruction_target, 0xABCD);
    }

    #[test]
    fn indexed_indirect() {
        let mut cpu = CPU::new([0x03].to_vec());
        cpu.x = 4;
        cpu.ram[0x07] = 0xCD;
        cpu.ram[0x08] = 0xAB;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::IndexedIndirect), 0);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.instruction_target, 0xABCD);
    }

    #[test]
    fn indirect_indexed() {
        // Without page crossed
        let mut cpu = CPU::new([0x03].to_vec());
        cpu.ram[0x03] = 0xCD;
        cpu.ram[0x04] = 0x00;
        cpu.y = 2;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::IndirectIndexed), 0);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.instruction_target, 0x00CF);

        // With page crossed
        let mut cpu = CPU::new([0x03].to_vec());
        cpu.ram[0x03] = 0xCD;
        cpu.ram[0x04] = 0xAB;
        cpu.y = 2;
        assert_eq!(cpu.compute_instruction_target(AddressingMode::IndirectIndexed), 1);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.instruction_target, 0xABCF);
    }

    //
    // Instructions
    //

    #[test]
    fn adc() {
        let mut cpu = CPU::new([
            0x69, 1,
            0x69, 1,
            0x69, 1,
        ].to_vec());

        cpu.a = 0;
        cpu.execute_next_instruction();
        assert_eq!(cpu.a, 1);
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.overflow);
        assert!(!cpu.flags.negative);

        cpu.a = 127;
        cpu.execute_next_instruction();
        assert_eq!(cpu.a, 128);
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.overflow);
        assert!(cpu.flags.negative);

        cpu.a = 255;
        cpu.execute_next_instruction();
        assert_eq!(cpu.a, 0);
        assert!(cpu.flags.carry);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.overflow);
        assert!(!cpu.flags.negative);
    }
}