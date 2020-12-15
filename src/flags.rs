use crate::util::BitOperations;

#[derive(Debug)]
pub struct CPUFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: u8,
    pub overflow: bool,
    pub negative: bool,
}

impl CPUFlags {
    pub fn new() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal_mode: false,
            break_command: 0,
            overflow: false,
            negative: false,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut result = 0;
        result.set_bit(0, self.carry);
        result.set_bit(1, self.zero);
        result.set_bit(2, self.interrupt_disable);
        result.set_bit(3, self.decimal_mode);
        result.set_bit(4, self.break_command.get_bit(0));
        result.set_bit(5, self.break_command.get_bit(1));
        result.set_bit(6, self.overflow);
        result.set_bit(7, self.negative);
        result
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            carry: byte.get_bit(0),
            zero: byte.get_bit(1),
            interrupt_disable: byte.get_bit(2),
            decimal_mode: byte.get_bit(3),
            break_command: {
                let mut break_command = 0;
                break_command.set_bit(0, byte.get_bit(4));
                break_command.set_bit(1, byte.get_bit(5));
                break_command
            },
            overflow: byte.get_bit(6),
            negative: byte.get_bit(7),
        }
    }
}
