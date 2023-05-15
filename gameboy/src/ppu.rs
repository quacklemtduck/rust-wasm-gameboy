use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, console};
use crate::memory::Memory;


const SCREEN_HEIGHT: usize = 144;
const SCREEN_WIDTH: usize = 160;
pub struct PPU {
    screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
    window_counter: u8,
}

impl PPU {
    pub fn new() -> PPU {
        PPU{
            screen: [0xff; SCREEN_WIDTH * SCREEN_HEIGHT * 4], 
            window_counter: 0}
    }

    pub fn advance_line(&mut self, mem: &mut Memory) {
        // println!("Advance");
        let mut ly = mem.read(0xff44);
        let lcdc = mem.read(0xff40);
        //Draw the line
        if ly < 144 {
            // If new graphics, parse them
            // if mem.new_graphics {
            //     // self.prepare_tile_map(mem);
            //     self.tile_cache.clear();
            //     // self.prepare_bg(mem);
            //     // self.draw_bg_tilemap(mem, bg_ctx);
            //     mem.new_graphics = false;
            // }

            self.draw_background_line(mem, ly, lcdc);
            self.draw_sprite_line(mem, ly, lcdc);
        } else {
            self.window_counter = 0;
        }

        ly = (ly + 1) % 154;
        mem.write(0xff44, ly);
    }

    // Gets the tile with index tile_index. Uses caching since tiles are usually used multiple times without changing
    fn get_tile(&mut self, mem: &mut Memory, tile_index: usize) -> Tile {
        let tile_option = mem.tile_cache[tile_index];
        match tile_option {
            Some(tile) => tile.clone(),
            None => self.parse_and_cache_tile(mem, tile_index),
        }
    }

    fn parse_and_cache_tile(&mut self, mem: &mut Memory, tile_index: usize) -> Tile {
        let mut tile = Tile::new();
        for x in 0..8 {
            let addr = 0x8000 + (tile_index as u16*16) + (x*2);
            println!("addr: {:#x}", addr);
            let a = mem.read(addr);
            let b = mem.read(addr + 1);
            let row = PPU::count_bits(a, b);
            println!("{:?}", row);
            for (j, n) in row.iter().enumerate() {
                tile.data[j + ((x as usize) * 8)] = *n;
                // self.tile_map[i as usize].data[j + ((x as usize) * 8)] = *n;
            }
        }
        mem.tile_cache[tile_index] = Some(tile.clone());
        //self.tile_cache.insert(tile_index, tile.clone());
        return tile;
    } 

    fn draw_background_line(&mut self, mem: &mut Memory, ly: u8, lcdc: u8) {
        
        if lcdc & 0x1 == 0 {
            for x in 0..144 {
                let screen_pos = ((x as usize) + ((ly as usize) * SCREEN_WIDTH)) * 4;
                let (r, g, b) = PPU::get_rgb(0);
                
                self.screen[screen_pos] = r;
                self.screen[screen_pos + 1] = g;
                self.screen[screen_pos + 2] = b;
                self.screen[screen_pos + 3] = 0xFF;
            }

            return
        }

        let data_area = lcdc & 0b10000 > 0;

        let map_area = lcdc & 0b1000 > 0;
        let window_area = lcdc & 0b1000000 > 0;
        
        let area_addr = if map_area {0x9C00} else {0x9800};
        let window_addr = if window_area {0x9C00} else {0x9800};

        let window_enable = lcdc & 0b100000 > 0;
        let wy = if window_enable {mem.read(0xFF4A) as i32} else {0};
        let wx = if window_enable && wy <= ly as i32 {mem.read(0xFF4B) as i32 - 7} else {0};

        let scy = mem.read(0xFF42) as usize;
        let scx = mem.read(0xFF43) as usize;

        let bg_palette = mem.read(0xFF47);
        let c_3 = bg_palette >> 6;
        let c_2 = (bg_palette >> 4) & 0b11;
        let c_1 = (bg_palette >> 2) & 0b11;
        let c_0 = bg_palette & 0b11;

        let y = scy + (ly as usize); 
        let mut x = scx;
        while x < (scx + 160 as usize){
            let tx = x % 256;
            let ty = y % 256;
            let t_index = ((tx / 8)) + ((ty / 8) * 32);
            
            let mut t_id = mem.read(area_addr + t_index as u16) as usize;

            // let mut t_id = self.bg[t_index] as usize;
            if !data_area && t_id < 128 {
                t_id = t_id + 256
            }
            let tile = self.get_tile(mem, t_id);
            //tile.print(t_id);

            for tile_x in (tx % 8)..8 {
                if x >= (scx + 160 as usize){
                    break
                }
                let tile_pos = tile_x + ((ty % 8) * 8);
                let (r, g, b) = self.get_color(tile.data[tile_pos], c_0, c_1, c_2, c_3);
                
                let screen_pos = ((x - scx) + ((ly as usize) * SCREEN_WIDTH)) * 4;

                self.screen[screen_pos] = r;
                self.screen[screen_pos + 1] = g;
                self.screen[screen_pos + 2] = b;
                self.screen[screen_pos + 3] = 0xFF;

                x += 1;
            }
        }

        if window_enable && ly as i32 >= wy && (wy < 144) && (wx < 166) {
            // let y = ly as i32 - wy;
            let y = self.window_counter as i32;

            self.window_counter += 1;
            let mut x = wx;
            while x < SCREEN_WIDTH as i32 {
                let t_index = (((x - wx) / 8) + ((y / 8) * 32)) as usize;
                
                let mut t_id = mem.read(window_addr + t_index as u16) as usize;
                
                // let mut t_id = self.window[t_index] as usize;
                if !data_area && t_id < 128 {
                    t_id = t_id + 256
                }
                let tile = self.get_tile(mem, t_id);
                for tile_x in 0..8 {
                    if x >= SCREEN_WIDTH as i32 {
                        break
                    }

                    if x < 0 { // Fixes inventory menu in Legend of Zelda
                        x += 1;
                        continue
                    }

                    let tile_pos = tile_x + ((y % 8) * 8);
                    let (r, g, b) = self.get_color(tile.data[tile_pos as usize], c_0, c_1, c_2, c_3);

                    let screen_pos = ((x as usize) + ((ly as usize) * SCREEN_WIDTH)) * 4;
                   
                    self.screen[screen_pos] = r;
                    self.screen[screen_pos + 1] = g;
                    self.screen[screen_pos + 2] = b;
                    self.screen[screen_pos + 3] = 0xFF;

                    x += 1;
                }
            }
        }

    }

    fn draw_sprite_line(&mut self, mem: &mut Memory, ly: u8, lcdc: u8) {
        
        if lcdc & 0b10 == 0 {
            return
        }
        
        // Looping through the sprites in reverse
        for i in (0..40).rev() {
            // Sprites use 4 bytes
            // Byte 0: Y Position
            // Byte 1: X Position
            // Byte 2: Tile Index
            // Byte 3: Attributes: Source: Pandocs
            //       Bit7   BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
            //       Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
            //       Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
            //       Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
            //       Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
            //       Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)

            let sprite_index = i * 4;
            let index = 0xFE00 + sprite_index; // Sprite table index

            

            let y = mem.read(index) as i32 - 16; // Offset by 16 pixels
            let x = mem.read(index + 1) as i32 - 8; // Offset by 8 pixels
            let mut tile_id = mem.read(index + 2);

            let sprite_height: i32 = if lcdc & 0b100 > 0 {
                tile_id = tile_id & !0x1; // Ignore the lower bit, enforced by the gameboy
                16
            } else {
                8
            };

            // If we should draw it
            if (ly as i32) >= y && (ly as i32) < y + sprite_height {
                
                let attributes = mem.read(index + 3);

                let flip_y = attributes & 0b1000000 > 0;
                let flip_x = attributes & 0b100000 > 0;

                let mut sprite_line = ly as i32 - y;

                if flip_y {
                    sprite_line = sprite_height - sprite_line - 1;
                }

                let tile = if sprite_line < 8 {self.get_tile(mem, tile_id as usize)} else {self.get_tile(mem, tile_id as usize + 1)};

                let palette = attributes & 0b10000 > 0;
                let palette_addr = if palette {0xFF49} else {0xFF48};
                let obj_palette = mem.read(palette_addr);
                let c_3 = obj_palette >> 6;
                let c_2 = (obj_palette >> 4) & 0b11;
                let c_1 = (obj_palette >> 2) & 0b11;
                let c_0 = obj_palette & 0b11;

                for tx in 0..8 {
                    let tile_x = if flip_x {7 - tx} else {tx};
                    let tile_pos = tile_x + ((sprite_line % 8) * 8);
                    let pixel = tile.data[tile_pos as usize];
                    if pixel == 0 {continue}
                    if x + tx >= SCREEN_WIDTH as i32 || x + tx < 0 {continue;}
                    let screen_pos = ((x + tx + (ly as i32 * SCREEN_WIDTH as i32)) * 4) as usize;
                    let (r, g, b) = self.get_color(pixel, c_0, c_1, c_2, c_3);

                    self.screen[screen_pos] = r;
                    self.screen[screen_pos + 1] = g;
                    self.screen[screen_pos + 2] = b;
                    self.screen[screen_pos + 3] = 0xFF;
                }
            }

        }
    }

    fn get_color(&self, index: u8, c_0: u8, c_1: u8, c_2: u8, c_3: u8) -> (u8, u8, u8) {
        match index {
            0 => PPU::get_rgb(c_0),
            1 => PPU::get_rgb(c_1),
            2 => PPU::get_rgb(c_2),
            3 => PPU::get_rgb(c_3),
            _ => (0,0,0)
        }
    }

    fn get_rgb(color: u8) -> (u8, u8, u8) {
        match color {
            0 => (0xe2, 0xf3, 0xe4),
            1 => (0x94, 0xe3, 0x44),
            2 => (0x46, 0x87, 0x8f),
            3 => (0x33, 0x2c, 0x50),
            _ => (0,0,0)
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
        // for i in 0..self.tile_map.len() {
        //     if (i/(160/8)*8) >= 144 {
        //         break;
        //     }
            
        //     self.paint_tile(i, ((i * 8) % 160) + 4, ((i * 8) / 160) * 8, true);
        // }
        //self.draw_background(mem);


        let data = ImageData::new_with_u8_clamped_array(Clamped(&mut self.screen), SCREEN_WIDTH as u32);
        match data {
            Ok(data) => {
                match ctx.put_image_data(&data, 0.0, 0.0) {
                    Ok(_) => {},
                    Err(_) => console::log_1(&"Error".into()),
                }
            },
            Err(e) => {
                console::log_1(&"Error".into());
                console::log_1(&e);
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    data: [u8; 64]
}

impl Tile {
    pub fn new() -> Tile {
        Tile{data: [0; 64]}
    }
}
