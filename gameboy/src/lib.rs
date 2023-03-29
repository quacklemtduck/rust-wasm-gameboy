extern crate core;

mod cpu;
mod memory;
mod cartridge;
mod ppu;

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
    iteration: u32,
    cnt: i32
}

#[wasm_bindgen]
impl GameBoy {
    pub fn new(data: Vec<u8>) -> GameBoy {
        let cart = Cartridge::New(data);
        let mem = Memory::new(Some(cart));
        GameBoy{ mem, cpu: CPU::new(), ppu: PPU::new(), iteration: 0, cnt: 0}
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
            if self.cnt < 0 {
                match stat & 0b11 {
                    0 => { // Going into either VBlank or Searching OAM
                        self.advance_line();
                        if self.mem.read(0xFF44) == 144{ //VBlank
                            stat = stat + 1;
                            if stat & 0b10000 > 0{
                                self.mem.write(0xFF0F, self.mem.read(0xFF0F) | 0b10);
                            }
                            self.ppu.draw(&mut self.mem, ctx)
                        } else {
                            stat = stat + 2;
                            if stat & 0b100000 > 0{
                                self.mem.write(0xFF0F, self.mem.read(0xFF0F) | 0b10);
                            }
                        }
                    },
                    1 => { // Going into Searching OAM
                        stat = stat + 1;
                        if stat & 0b100000 > 0{
                            self.mem.write(0xFF0F, self.mem.read(0xFF0F) | 0b10);
                        }
                    }
                }
            }
            self.mem.write(0xFF41, stat);
        }

        loop {
            self.iteration += 1;
            self.step();
            if self.iteration % 453 == 0{
                self.advance_line()
            }
            if self.mem.read(0xFF44) == 144 {
                self.ppu.draw(&mut self.mem, ctx)
            }
            if self.iteration >= 69905 {
                self.iteration = self.iteration - 69905;
                break
            }
        }
        // self.ppu.draw(&mut self.mem, ctx);
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

