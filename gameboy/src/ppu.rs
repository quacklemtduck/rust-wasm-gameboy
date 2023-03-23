use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, console};
use crate::memory::Memory;

pub struct PPU {
    tile_map: [Tile; 384]
}

impl PPU {
    pub fn new() -> PPU {
        PPU{tile_map: [Tile::new(); 384]}
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

    pub fn draw(&mut self, ctx: &CanvasRenderingContext2d) {
        for (i, t) in self.tile_map.iter().enumerate() {
            if (i/(160/8)*8) >= 144 {
                break;
            }
            let mut img: Vec<u8> = Vec::new();
            for (p, v) in t.data.iter().enumerate(){
                match *v {
                    0 => {

                        img.push(0xe2);
                        img.push(0xf3);
                        img.push(0xe4);
                        img.push(255);
                    }
                    1 => {
                        img.push(0x94);
                        img.push(0xe3);
                        img.push(0x44);
                        img.push(255);
                    }
                    2 => {
                        img.push(0x46);
                        img.push(0x87);
                        img.push(0x8f);
                        img.push(255);
                    }
                    3 => {
                        img.push(0x33);
                        img.push(0x2c);
                        img.push(0x50);
                        img.push(255);
                    }
                    _ => {
                        img.push(0x00);
                        img.push(0x00);
                        img.push(0x00);
                        img.push(255);
                    }
                }
            }
            console::log_1(&format!("Size: {}", img.len()).into());
            let data = ImageData::new_with_u8_clamped_array(Clamped(&mut img), 8);
            match data {
                Ok(data) => {
                    match ctx.put_image_data(&data, (i%(160/8) * 8) as f64, (i/(160/8)*8) as f64) {
                        Ok(_) => {},
                        Err(_) => console::log_1(&"Error".into()),
                    }
                },
                Err(e) => {
                    console::log_1(&e);
                },
            }
        }

        // img = Vec::new();
        // for _ in 0..200 {
        //     for _ in 0..200 {
        //         img.push(0x33);
        //         img.push(0x2c);
        //         img.push(0x50);
        //         img.push(255);
        //     }
        // }

        
        
        
        
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