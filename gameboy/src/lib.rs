extern crate core;

mod cpu;
mod memory;
mod cartridge;
mod ppu;
mod joypad;

use joypad::Joypad;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::console;
use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::memory::Memory;
use crate::ppu::PPU;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const LCDC: u16 = 0xFF40;

#[wasm_bindgen]
pub struct GameBoy {
    mem: Memory,
    cpu: CPU,
    ppu: PPU,
    joypad: Joypad,
    iteration: u32,
    cnt: i32
}

#[wasm_bindgen]
impl GameBoy {
    pub fn new(data: Vec<u8>) -> GameBoy {
        let cart = Cartridge::new(data);
        let mem = Memory::new(Some(cart));
        GameBoy{ mem, cpu: CPU::new(), ppu: PPU::new(), iteration: 0, cnt: 0, joypad: Joypad::new()}
    }

    pub fn start(&mut self, ctx: &CanvasRenderingContext2d) {
        self.cpu.simulate_bootloader();
        self.mem.simulate_bootloader();
        self.cnt = 80;

        ctx.fill_rect(0.0, 0.0, 100.0, 100.0);

        // for i in 0..1000000 {
        //     self.step();
        //     self.advance_line();
        //     if i % 50000 == 0 {
        //         self.ppu.prepare_tile_map(&mut self.mem);
        //         self.ppu.draw(ctx);
        //     }
        // }
    }

     pub fn run(&mut self, ctx: &CanvasRenderingContext2d) {

        loop {
            self.cnt -= self.step() as i32;
            let mut stat = self.mem.read(0xFF41);
            //console::log_1(&format!("Mode: {} Cnt: {}", stat & 0b11, self.cnt).into());
            if self.cnt < 0 {
                match stat & 0b11 {
                    0 => { // Going into either VBlank or Searching OAM
                        if self.mem.read(0xFF44) >= 144{ //VBlank
                            stat = stat + 1;
                            self.cnt += 4560;
                            if stat & 0b10000 > 0{
                                self.mem.write(0xFF0F, self.mem.read(0xFF0F) | 0b10);
                            }
                            self.mem.write(0xFF0F, self.mem.read(0xFF0F) | 0b1);
                            //console::log_1(&format!("Draw {}", self.mem.read(0xFF44)).into());
                            self.ppu.draw(&mut self.mem, ctx)
                        } else {
                            stat = stat + 2;
                            self.cnt += 80;
                            if stat & 0b100000 > 0{
                                self.mem.write(0xFF0F, self.mem.read(0xFF0F) | 0b10);
                            }
                        }
                    },
                    1 => { // Going into Searching OAM, end of frame
                        stat = stat + 1;
                        self.cnt += 80;
                        if stat & 0b100000 > 0{
                            self.mem.write(0xFF0F, self.mem.read(0xFF0F) | 0b10);
                        }
                        self.mem.write(0xFF41, stat);
                        return;
                    },
                    2 => { // Going into Generating picture
                        stat = stat + 1;
                        self.cnt += 168;
                    },
                    3 => {
                        self.advance_line();
                        stat = stat - 3;
                        self.cnt += 208;
                        if stat & 0b1000 > 0{
                            self.mem.write(0xFF0F, self.mem.read(0xFF0F) | 0b10);
                        }
                    },
                    _ => console::error_1(&"Unreachable mode".into())

                }
            } else if stat & 0b11 == 1 {
                if self.mem.read(0xFF44) >= 144 {
                    self.advance_line()
                }
            }
            self.mem.write(0xFF41, stat);
        }
        // self.ppu.draw(&mut self.mem, ctx);
     }

     pub fn set_joypad_state(&mut self, up: i32, right: i32, down: i32, left: i32, a: i32, b: i32, select: i32, start: i32) {
        self.joypad.set_joypad_state(up, right, down, left, a, b, select, start, &mut self.mem)
     }

    pub fn step(&mut self) -> u8 {
        self.cpu.run(&mut self.mem)
    }

    pub fn advance_line(&mut self) {
        self.ppu.advance_line(&mut self.mem);
    }

    #[wasm_bindgen(skip)]
    pub fn print(&mut self) {
        self.cpu.print();
        self.mem.print();
        self.ppu.prepare_tile_map(&mut self.mem);
        self.ppu.print_tile();
    }
}

