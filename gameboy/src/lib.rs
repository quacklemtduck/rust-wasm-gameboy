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
}

#[wasm_bindgen]
impl GameBoy {
    pub fn new(data: Vec<u8>) -> GameBoy {
        let cart = Cartridge::New(data);
        let mem = Memory::new(Some(cart));
        GameBoy{ mem, cpu: CPU::new(), ppu: PPU::new() }
    }

    pub fn start(&mut self, ctx: &CanvasRenderingContext2d) {
        self.cpu.simulate_bootloader();
        self.mem.simulate_bootloader();

        ctx.fill_rect(0.0, 0.0, 100.0, 100.0);

        for i in 0..1000000 {
            self.step();
            self.advance_line();
            if i % 50000 == 0 {
                console::log_1(&"Draw".into());
                self.ppu.prepare_tile_map(&mut self.mem);
                self.ppu.draw(ctx);
            }

        }

    }

    pub fn step(&mut self) {
        self.cpu.run(&mut self.mem);
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

