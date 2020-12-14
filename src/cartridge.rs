use std::fs::File;
use std::io::{Error, Read};
use std::ops::RangeInclusive;
use std::path::Path;

use crate::util::BitOperations;

#[derive(Debug)]
pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Cartridge {
    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Self {
        let mut buffer = {
            let mut file = File::open(filepath).unwrap();
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            buffer
        };
        let (header, data) = buffer.split_at(16);

        if !header.starts_with(b"NES\x1A") {
            return panic!("Not a NES file.");
        }

        let prg_rom_size = {
            let lsb = header[4];
            let msb = header[9].get_bits(0..=3);

            if msb == 0xF {
                let multiplier = lsb.get_bits(0..=1);
                let exponent = lsb.get_bits(2..=7);
                2usize.pow(exponent as u32) * (multiplier as usize * 2 + 1)
            } else {
                let mut size = 0u16;
                size.set_bits(0..=7, lsb as u16);
                size.set_bits(8..=11, msb as u16);
                size as usize * 16 * 1024
            }
        };

        let chr_rom_size = {
            let lsb = header[5];
            let msb = header[9].get_bits(4..=7);

            if msb == 0xF {
                let multiplier = lsb.get_bits(0..=1);
                let exponent = lsb.get_bits(2..=7);
                2usize.pow(exponent as u32) * (multiplier as usize * 2 + 1)
            } else {
                let mut size = 0u16;
                size.set_bits(0..=7, lsb as u16);
                size.set_bits(8..=11, msb as u16);
                size as usize * 8 * 1024
            }
        };

        let mapper_number = {
            let mut n = 0u16;
            n.set_bits(0..=3, header[6].get_bits(4..=7) as u16);
            n.set_bits(4..=7, header[7].get_bits(4..=7) as u16);
            n.set_bits(8..=11, header[8].get_bits(0..=3) as u16);
            n
        };

        let is_trainer_present = header[6].get_bit(2);

        // Skip useless trainer
        let data = if is_trainer_present {
            &data[512..]
        } else {
            &data
        };

        let (prg_rom, cdr) = data.split_at(prg_rom_size);
        let (chr_rom, _cdr) = cdr.split_at(chr_rom_size);

        Self {
            prg_rom: prg_rom.to_vec(),
            chr_rom: chr_rom.to_vec(),
        }
    }
}