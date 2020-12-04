#[derive(Debug)]
pub struct CPUFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    _decimal_mode: bool,
    _break: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl CPUFlags {
    pub fn new() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt_disable: false,
            _decimal_mode: false,
            _break: false,
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
}