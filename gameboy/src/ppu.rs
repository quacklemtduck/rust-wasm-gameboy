use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, console};
use crate::memory::Memory;


const SCREEN_HEIGHT: usize = 144;
const SCREEN_WIDTH: usize = 160;
const PIXELS: usize = 160 * 144;
pub struct PPU {
    tile_map: [Tile; 384],
    screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
}

impl PPU {
    pub fn new() -> PPU {
        PPU{tile_map: [Tile::new(); 384], screen: [0xff; SCREEN_WIDTH * SCREEN_HEIGHT * 4]}
    }

    pub fn advance_line(&self, mem: &mut Memory) {
        // println!("Advance");
        let mut ly = mem.read(0xff44);
        ly = (ly + 1) % 154;
        mem.write(0xff44, ly);
    }

    pub fn prepare_tile_map(&mut self, mem: &mut Memory){
        for i in 0..384 {
            for x in 0..8 {
                let addr = 0x8000 + (i*16) + (x*2);
                println!("addr: {:#x}", addr);
                let a = mem.read(addr);
                let b = mem.read(addr + 1);
                let row = PPU::count_bits(a, b);
                println!("{:?}", row);
                for (j, n) in row.iter().enumerate() {
                    self.tile_map[i as usize].data[j + ((x as usize) * 8)] = *n;
                }
            }
        }
    }

    pub fn print_tile(&self) {
        for t in 0..self.tile_map.len() {
            let tile = self.tile_map[t];
            println!("Tile {:#x} {:?}", t, tile);
            for i in 0..8 {
                for j in 0..8 {
                    let symbol = match tile.data[(i * 8) + j] {
                        0 => " ",
                        1 => "░",
                        2 => "▒",
                        3 => "▓",
                        _ => ""
                    };
                    print!("{} ", symbol);
                }
                println!();
            }
            println!();
        }
    }

    fn count_bits(a: u8, b: u8) -> [u8; 8] {
        let mut result = [0u8; 8];
        let mut mask = 1u8;
        for i in (0..8).rev() {
            if (a & mask) != 0 {
                result[i] += 1;
            }
            if (b & mask) != 0 {
                result[i] += 2;
            }
            mask <<= 1;
        }
        result
    }

    fn paint_tile(&mut self, tile_id: usize, x: usize, y: usize) {
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return
        }
        let tile = self.tile_map[tile_id];
        for (px, &p) in tile.data.iter().enumerate() {
            if x + (px % 8) >= SCREEN_WIDTH || y + (px / 8) >= SCREEN_HEIGHT {
                return
            }
            let pos = ((SCREEN_WIDTH * y) + x + (px % 8) + ((px / 8) * SCREEN_WIDTH)) * 4;
            match p {
                0 => {
                    self.screen[pos] = 0xe2;
                    self.screen[pos + 1] = 0xf3;
                    self.screen[pos + 2] = 0xe4;
                    self.screen[pos + 3] = 255;
                }
                1 => {
                    self.screen[pos] = 0x94;
                    self.screen[pos + 1] = 0xe3;
                    self.screen[pos + 2] = 0x44;
                    self.screen[pos + 3] = 255;
                }
                2 => {
                    self.screen[pos] = 0x46;
                    self.screen[pos + 1] = 0x87;
                    self.screen[pos + 2] = 0x8f;
                    self.screen[pos + 3] = 255;
                }
                3 => {
                    self.screen[pos] = 0x33;
                    self.screen[pos + 1] = 0x2c;
                    self.screen[pos + 2] = 0x50;
                    self.screen[pos + 3] = 255;
                }
                _ => {}
            }
        }
    }

    pub fn draw(&mut self, ctx: &CanvasRenderingContext2d) {
        for i in 0..self.tile_map.len() {
            if (i/(160/8)*8) >= 144 {
                break;
            }
            
            self.paint_tile(i, (i * 8) % 160, ((i * 8) / 160) * 8);
        }


        let data = ImageData::new_with_u8_clamped_array(Clamped(&mut self.screen), SCREEN_WIDTH as u32);
        match data {
            Ok(data) => {
                match ctx.put_image_data(&data, 0.0, 0.0) {
                    Ok(_) => {},
                    Err(_) => console::log_1(&"Error".into()),
                }
            },
            Err(e) => {
                console::log_1(&"What".into());
                console::log_1(&e);
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    data: [u8; 64]
}

impl Tile {
    pub fn new() -> Tile {
        Tile{data: [0; 64]}
    }
}