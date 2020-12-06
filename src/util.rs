#[inline]
pub fn page_of(address: u16) -> u8 {
    (address >> 8) as u8
}

pub trait BitOperations {
    fn get_bit(&self, index: u8) -> bool;
    fn set_bit(&mut self, index: u8, value: bool);
}

impl BitOperations for u8 {
    fn get_bit(&self, index: u8) -> bool {
        (self & (1 << index)) != 0
    }

    fn set_bit(&mut self, index: u8, value: bool) {
        let clear_bit = *self & !(1 << index);
        *self = clear_bit | ((value as u8) << index);
    }
}
