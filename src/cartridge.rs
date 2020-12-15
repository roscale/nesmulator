use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::mapper::{Mapper, Mapper0};
use crate::util::BitOperations;

#[derive(Debug)]
pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Self {
        let buffer = {
            let mut file = File::open(filepath).unwrap();
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            buffer
        };
        let (header, data) = buffer.split_at(16);

        if !header.starts_with(b"NES\x1A") {
            panic!("Not a NES file.");
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
            mapper: {
                if mapper_number == 0 {
                    Box::new(Mapper0::new(prg_rom.len()))
                } else {
                    unimplemented!();
                }
            },
        }
    }
}

impl std::ops::Index<u16> for Cartridge {
    type Output = u8;

    fn index(&self, address: u16) -> &Self::Output {
        &self.prg_rom[self.mapper.address(address) as usize]
    }
}

impl std::ops::IndexMut<u16> for Cartridge {
    fn index_mut(&mut self, address: u16) -> &mut Self::Output {
        &mut self.prg_rom[self.mapper.address(address) as usize]
    }
}
