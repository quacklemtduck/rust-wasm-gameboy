use crate::cartridge::CType::*;

pub struct Cartridge {
    pub data: Vec<u8>,
    c_type: CType,
    // Size in KiB
    rom_size: u16,
    ram_size: u16,

    ram_enable: bool,
    rom_bank: u8

}

pub enum CType {
    Rom,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Self {
        let c_type = match data[0x0147] {
            0x00 => Rom,
            0x01 => Mbc1,
            0x02 => Mbc1Ram,
            0x03 => Mbc1RamBattery,
            _ => {
                panic!("Rom type not implemented!")
            },
        };
        let rom_size = 32 * (1 << data[0x0148]);
        let ram_size = match data[0x0149] {
            0x00 => 0,
            0x02 => 8,
            0x03 => 32,
            0x04 => 128,
            0x05 => 64,
            _ => 0
        };

        return Cartridge{
            data,
            c_type,
            rom_size,
            ram_size,
            ram_enable: false,
            rom_bank: 1
        }
    }


    pub fn read(&self, loc: u16) -> u8{
        return self.data[loc as usize]
    }

    pub fn read_16(&self, loc: u16) -> u16 {
        let low = self.read(loc);
        let high = self.read(loc + 1);
        return ((high as u16) << 8) | low as u16;
    }

    pub fn write(&mut self, loc: u16, val: u8){
        if loc < 2000 {
            if val & 0x0F == 0x0A {
                self.ram_enable = true;
            }else{
                self.ram_enable = false;
            }
        }

        if loc < 0x8000 {
            return
        }
        self.data[loc as usize] = val
    }

    pub fn write_16(&mut self, loc: u16, val: u16) {
        let low = (val & 0xff) as u8;
        let high = (val >> 8) as u8;
        self.write(loc, low);
        self.write(loc + 1, high);
    }

}