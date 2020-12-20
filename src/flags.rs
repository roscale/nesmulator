use crate::util::BitOperations;

#[derive(Debug)]
pub struct CPUFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal_mode: bool,
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
        result.set_bit(4, false); // To be set by the interrupt
        result.set_bit(5, true); // Unused, always true
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
            overflow: byte.get_bit(6),
            negative: byte.get_bit(7),
        }
    }
}
