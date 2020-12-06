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

    #[inline]
    pub fn modify_zero_flag(&mut self, value: u8) {
        self.zero = value == 0;
    }

    #[inline]
    pub fn modify_negative_flag(&mut self, value: u8) {
        self.negative = (value & 0b1000_0000) != 0;
    }

    pub fn to_byte(&self) -> u8 {
        ((self.carry as u8) << 0) |
            ((self.zero as u8) << 1) |
            ((self.interrupt_disable as u8) << 2) |
            ((self.decimal_mode as u8) << 3) |
            ((self.break_command & 0x01) << 4) |
            ((self.break_command & 0x02) << 4) |
            ((self.overflow as u8) << 6) |
            ((self.negative as u8) << 7)
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            carry: (byte & (1 << 0)) != 0,
            zero: (byte & (1 << 1)) != 0,
            interrupt_disable: (byte & (1 << 2)) != 0,
            decimal_mode: (byte & (1 << 3)) != 0,
            break_command: {
                let mut break_command = ((byte & (1 << 4)) != 0) as u8;
                break_command |= (((byte & (1 << 5)) != 0) as u8) << 1;
                break_command
            },
            overflow: (byte & (1 << 6)) != 0,
            negative: (byte & (1 << 7)) != 0,
        }
    }
}