extern crate core;

mod cpu;
mod memory;
mod cartridge;
mod ppu;

use wasm_bindgen::prelude::*;
use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::memory::Memory;
use crate::ppu::PPU;

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

    pub fn start(&mut self) {
        self.cpu.simulate_bootloader();
        self.mem.simulate_bootloader();
    }

    pub fn step(&mut self) {
        self.cpu.run(&mut self.mem)
    }

    pub fn advance_line(&mut self) {
        self.ppu.advance_line(&mut self.mem);
    }

    pub fn print(&mut self) {
        self.cpu.print();
        self.ppu.prepare_tile_map(&mut self.mem);
        self.ppu.print_tile();
    }
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, gameboy!");
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a * 2 + b * 2
}

#[wasm_bindgen]
pub fn dec(a: i32, b: i32) -> i32 {
    a - b - b
}