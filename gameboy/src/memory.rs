use crate::{cartridge::Cartridge, joypad::Joypad, state::{InitialState, FinalState}, ppu::Tile};

pub struct Memory {
    pub mem: [u8; 0x10000],
    pub cart: Cartridge,
    pub new_graphics: bool,
    pub joypad: Joypad,
    test_mode: bool,
    pub tile_cache: [Option<Tile>; 384]
}

impl Memory {
    // If cart is None, then memory is put into test mode with no limitations
    pub fn new(cart: Option<Cartridge>) -> Memory {
        let mut test_mode = false;
        let c: Cartridge = match cart {
            None => {
                test_mode = true;
                Cartridge::new(vec![0; 1024 * 32], "test".to_string())},
            Some(x) => x
        };
        let tile_cache: [Option<Tile>; 384] = [None; 384];
        return Memory{mem: [0; 0x10000], cart: c, new_graphics: true, joypad: Joypad::new(), test_mode, tile_cache: tile_cache }
    }

    pub fn load_state(&mut self, state: &InitialState){
        for v in &state.ram {
            let addr = v[0];
            let val = v[1] as u8;
            self.write(addr, val);
        }
    }

    pub fn compare_state(&self, state: &FinalState) -> Result<(), String> {
            for v in &state.ram {
                let addr = v[0];
                let expected = v[1] as u8;
                let actual = self.read(addr);

                if expected != actual {
                    return Err(format!("Expected at mem {:#x}: {:#x}, actual: {:#x}", addr, expected, actual))
                }
            }
        return Ok(())
    }

    pub fn print(&self) {
        for i in 0..(0xFFFFu16 / 16) {
            print!("0x{:04x} | ", i * 16);
            for a in 0..16u16 {
                print!("0x{:02x} ", self.read((i * 16) + a));
            }
            println!();
        }
    }

    pub fn read(&self, loc: u16) -> u8{
        if self.test_mode {
            return self.mem[loc as usize]
        }

        if loc < 0x8000 || (0xA000 <= loc && loc <= 0xBFFF){
            return self.cart.read(loc);
        }

        // JoyPad
        if loc == 0xFF00 {
            return self.joypad.get_joypad_state();
        }
        // println!("Read: 0x{:02x}", v);
        return self.mem[loc as usize]
    }

    pub fn read_16(&self, loc: u16) -> u16 {
        let low = self.read(loc);
        let high = self.read(loc + 1);
        return ((high as u16) << 8) | low as u16;
    }

    // pub fn read_signed(&self, loc: u16) -> i8 {
    //     let val = self.mem[loc as usize];
    //     return val as i8;
    // }

    pub fn write(&mut self, loc: u16, val: u8){
        if self.test_mode {
            self.mem[loc as usize] = val;
            return
        }

        if loc < 0x8000 || (0xA000 <= loc && loc <= 0xBFFF) {
            self.cart.write(loc, val);
            return
        }

        // New Tile data
        if loc >= 0x8000 && loc <= 0x97FF {
            let tile_id = (loc - 0x8000) / 16;
            self.tile_cache[tile_id as usize] = None;
        }

        // JoyPad
        if loc == 0xFF00 {
            self.joypad.update_joypad(val)
        }

        if loc == 0xFF46 {
            let source = (val as usize) << 8;
            for i in 0..0x100 {
                self.mem[0xFE00 + i] = self.mem[source + i];
            }
        }

        // Compare LY and LYC
        if loc == 0xFF44 {
            if val == self.mem[0xFF45] {
                self.mem[0xFF41] = self.mem[0xFF41] | 0x4;
                // Interrupt
                if self.mem[0xFF41] & 0b01000000 > 0{
                    self.mem[0xFF0F] = self.mem[0xFF0F] | 0x2 // Interrupt
                }
            } else {
                self.mem[0xFF41] = self.mem[0xFF41] & 0xFB;
            }
        }
        self.mem[loc as usize] = val
    }

    pub fn write_16(&mut self, loc: u16, val: u16) {
        let low = (val & 0xff) as u8;
        let high = (val >> 8) as u8;
        self.write(loc, low);
        self.write(loc + 1, high);
    }

    pub fn simulate_bootloader(&mut self) {
        self.write(0xff00, 0xcf);
        self.write(0xff01, 0x00);
        self.write(0xff02, 0x7e);
        self.write(0xff04, 0xab);
        self.write(0xff05, 0x00);
        self.write(0xff06, 0x00);
        self.write(0xff07, 0xf8);
        self.write(0xff0f, 0xe1);
        self.write(0xff10, 0x80);
        self.write(0xff11, 0xbf);
        self.write(0xff12, 0xf3);
        self.write(0xff13, 0xff);
        self.write(0xff14, 0xbf);
        self.write(0xff16, 0x3f);
        self.write(0xff17, 0x00);
        self.write(0xff18, 0xff);
        self.write(0xff19, 0xbf);
        self.write(0xff1a, 0x7f);
        self.write(0xff1b, 0xff);
        self.write(0xff1c, 0x9f);
        self.write(0xff1d, 0xff);
        self.write(0xff1e, 0xbf);
        self.write(0xff20, 0xff);
        self.write(0xff21, 0x00);
        self.write(0xff22, 0x00);
        self.write(0xff23, 0xbf);
        self.write(0xff24, 0x77);
        self.write(0xff25, 0xf3);
        self.write(0xff26, 0xf1);
        self.write(0xff40, 0x91);
        self.write(0xff41, 0x86);
        self.write(0xff42, 0x00);
        self.write(0xff43, 0x00);
        self.write(0xff44, 0x00);
        self.write(0xff45, 0x00);
        self.write(0xff46, 0xff); // ??
        self.write(0xff47, 0xfc);
        self.write(0xff4a, 0x00);
        self.write(0xff4b, 0x00);
        self.write(0xff4d, 0xff);
        self.write(0xff4f, 0xff);
        self.write(0xff51, 0xff);
        self.write(0xff52, 0xff);
        self.write(0xff53, 0xff);
        self.write(0xff54, 0xff);
        self.write(0xff55, 0xff);
        self.write(0xff56, 0xff);
        self.write(0xff68, 0xff);
        self.write(0xff69, 0xff);
        self.write(0xff6a, 0xff);
        self.write(0xff6b, 0xff);
        self.write(0xff70, 0xff);
        self.write(0xffff, 0x00);
    }


    pub fn set_joypad_state(&mut self, up: i32, right: i32, down: i32, left: i32, a: i32, b: i32, select: i32, start: i32) {
        let request_interrupt = self.joypad.set_joypad_state(up, right, down, left, a, b, select, start);

        if request_interrupt {
            self.mem[0xFF0F] = self.mem[0xFF0F] | 0b10000;
        }
    }
}