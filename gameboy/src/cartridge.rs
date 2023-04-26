use web_sys::console;

use crate::{cartridge::CType::*, save};

pub struct Cartridge {
    name: String,
    pub data: Vec<u8>,
    pub ram: Vec<u8>,
    c_type: CType,
    // Size in KiB
    rom_size: u16,
    ram_size: u16,

    ram_enable: bool,
    ram_bank: u8,
    rom_bank: u8,
    num_banks: u8,

    banking_mode: u8,

}

pub enum CType {
    Rom,
    Mbc1,
    Mbc3,
}

impl Cartridge {
    pub fn new(data: Vec<u8>, name: String) -> Self {
        let c_type = match data[0x0147] {
            0x00 => Rom,
            0x01 | 0x02 | 0x03 => Mbc1,
            0x11 | 0x12 | 0x13 => Mbc3, // No support for timers
            _ => {
                panic!("Rom type not implemented!")
            },
        };
        let rom_size = 32 * (1 << data[0x0148]);
        let num_banks = data[0x0148] + 1;
        let ram_size = match data[0x0149] {
            0x00 => 0,
            0x02 => 8,
            0x03 => 32,
            0x04 => 128,
            0x05 => 64,
            _ => 0
        };

        let ram = match save::get_item(&name) {
            Some(val) => val,
            None => vec![0; ram_size as usize * 1024],
        };

        //let ram: Vec<u8> = vec![0; ram_size as usize * 1024];


        return Cartridge{
            name,
            data,
            c_type,
            rom_size,
            ram_size,
            ram,
            ram_enable: false,
            rom_bank: 1,
            num_banks,
            ram_bank: 0,
            banking_mode: 0
        }
    }


    pub fn read(&self, loc: u16) -> u8{

        if (loc >= 0x4000) && (loc <= 0x7fff) {
            //console::log_1(&format!("Rom {:#x} {} {}", self.rom_bank, self.num_banks, self.rom_size).into());
            let new_address = loc as usize - 0x4000; //Setting it to 0, in case the bank is 0
            return self.data[new_address + (self.rom_bank as usize * 0x4000)];
        }
        if (loc >= 0xA000) && (loc <= 0xbfff) {
            let new_address = loc as usize - 0xA000; //Setting it to 0, in case the bank is 0
            let val = self.ram[new_address + (self.ram_bank as usize * 0x4000)];
            // console::log_1(&format!("Ram write enable: {}, addr: {:#x}, bank: {}, value: {}", self.ram_enable, new_address, self.ram_bank, val).into());
            return val;
        }

        return self.data[loc as usize]
    }

    pub fn read_16(&self, loc: u16) -> u16 {
        let low = self.read(loc);
        let high = self.read(loc + 1);
        return ((high as u16) << 8) | low as u16;
    }

    pub fn write(&mut self, loc: u16, val: u8){
        if loc < 0x2000 {
            if val & 0x0F == 0x0A {
                self.ram_enable = true;
            }else{
                self.ram_enable = false;
                // Save data
                save::set_item(&self.name, &self.ram).unwrap();
            }
        } else if loc < 0x4000 {
            match self.c_type {
                Rom | Mbc1 => {
                    let mut bank = if val == 0 {0x01} else {val};
                    bank = bank & ((1 << self.num_banks) - 1);

                    self.rom_bank = bank;
                },
                Mbc3 => {
                    self.rom_bank = val
                },
            }
            
        } else if loc < 0x6000 {
            self.ram_bank = val & 0x03;
        } else if loc < 0x8000 {
            self.banking_mode = val & 0x1;
        }
        

        if loc < 0x8000 {
            return
        }

        if (loc >= 0xA000) && (loc <= 0xBFFF) {
            
            if self.ram_enable {
                let new_address = loc as usize - 0xA000; //Setting it to 0, in case the bank is 0
                // console::log_1(&format!("Ram enable: {}, addr: {:#x}, bank: {}", self.ram_enable, new_address, self.ram_bank).into());
                self.ram[new_address + (self.ram_bank as usize * 0x4000)] = val;
            }

            return
        }
        //self.data[loc as usize] = val
    }

    pub fn write_16(&mut self, loc: u16, val: u16) {
        let low = (val & 0xff) as u8;
        let high = (val >> 8) as u8;
        self.write(loc, low);
        self.write(loc + 1, high);
    }

}
