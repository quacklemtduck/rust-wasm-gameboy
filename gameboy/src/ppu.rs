use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, console};
use crate::memory::Memory;


const SCREEN_HEIGHT: usize = 144;
const SCREEN_WIDTH: usize = 160;
const PIXELS: usize = 160 * 144;
pub struct PPU {
    tile_map: [Tile; 384],
    screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],

    bg: [u8; 32 * 32]
}

impl PPU {
    pub fn new() -> PPU {
        PPU{tile_map: [Tile::new(); 384], screen: [0xff; SCREEN_WIDTH * SCREEN_HEIGHT * 4], bg: [0; 32 * 32]}
    }

    pub fn advance_line(&mut self, mem: &mut Memory) {
        // println!("Advance");
        let mut ly = mem.read(0xff44);
        let lcdc = mem.read(0xff40);
        //Draw the line
        if ly < 144 {
            // If new graphics, parse them
            if mem.new_graphics {
                console::log_1(&"New".into());
                self.prepare_tile_map(mem);
                self.prepare_bg(mem);
                // self.draw_bg_tilemap(mem, bg_ctx);
                mem.new_graphics = false;
            }

            self.draw_background_line(mem, ly);
            self.draw_sprite_line(mem, ly, lcdc);

        }

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

    pub fn prepare_bg(&mut self, mem: &mut Memory){
        let lcdc = mem.read(0xFF40);
        let map_area = lcdc & 0b1000 > 0;
        
        let area_addr = if map_area {0x9C00} else {0x9800};
        for i in 0..1024 {
            self.bg[i] = mem.read(area_addr + i as u16);
        }

    }

    fn draw_bg_tilemap(&mut self, mem: &mut Memory, bgCtx: &CanvasRenderingContext2d){
        let lcdc = mem.read(0xFF40);
        let data_area = lcdc & 0b10000 > 0;
        let mut screen: [u8; 256 * 256 * 4] = [0; 256 * 256 * 4];
        for i in 0..self.bg.len() {
            let mut t_id = self.bg[i] as usize;
            if !data_area && t_id < 128 {
                t_id = t_id + 256
            }
            self.paint_tile(&mut screen, 256, 256, t_id, (i % 32) * 8, (i / 32) * 8, false);
        }

        let data = ImageData::new_with_u8_clamped_array(Clamped(&mut screen), 256 as u32);
        match data {
            Ok(data) => {
                match bgCtx.put_image_data(&data, 0.0, 0.0) {
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

    fn draw_background_line(&mut self, mem: &mut Memory, ly: u8) {
        //console::log_1(&format!("Drawing: {}", ly).into());
        let lcdc = mem.read(0xFF40);
        let data_area = lcdc & 0b10000 > 0;

        let scy = mem.read(0xFF42) as usize;
        let scx = mem.read(0xFF43) as usize;

        let bg_palette = mem.read(0xFF47);
        let c_3 = bg_palette >> 6;
        let c_2 = (bg_palette >> 4) & 0b11;
        let c_1 = (bg_palette >> 2) & 0b11;
        let c_0 = bg_palette & 0b11;

        let y = scy + (ly as usize); 
        let mut x = scx;
        while x < (scx + 160){
            let tx = x % 256;
            let ty = y % 256;
            let t_index = ((tx / 8)) + ((ty / 8) * 32);
            let mut t_id = self.bg[t_index] as usize;
            if !data_area && t_id < 128 {
                t_id = t_id + 256
            }
            let tile = self.tile_map[t_id];
            //tile.print(t_id);

            for tile_x in (tx % 8)..8 {
                if x >= (scx + 160){
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

    }

    fn draw_sprite_line(&mut self, mem: &mut Memory, ly: u8, lcdc: u8) {
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

            let spriteIndex = i * 4;
            let index = 0xFE00 + spriteIndex; // Sprite table index

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
                // if i == 39 {
                //     console::log_1(&format!("Drawing sprite with tile {} at X: {} Y: {} {:#x}", tile_id, x, y, index + 1).into());
                // }
                
                let attributes = mem.read(index + 3);

                let flip_y = attributes & 0b1000000 > 0;
                let flip_x = attributes & 0b100000 > 0;

                let mut sprite_line = ly as i32 - y;

                if flip_y {
                    sprite_line = sprite_height - sprite_line - 1;
                }

                let tile = if sprite_line < 8 {self.tile_map[tile_id as usize]} else {self.tile_map[tile_id as usize + 1]};

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

    fn paint_tile(&mut self, screen: &mut [u8], width: usize, height: usize,tile_id: usize, x: usize, y: usize, wrap: bool) {
        //console::log_1(&format!("tile: {}", tile_id).into());
        if !wrap && (x >= width || y >= height) {
            return
        }
        let tile = self.tile_map[tile_id];
        for (px, &p) in tile.data.iter().enumerate() {
            if !wrap && (x + (px % 8) >= width || y + (px / 8) >= height) {
                continue
            }
            let pos = if wrap {
                ((((y + (px / 8)) % height) * width) + ((x + (px % 8)) % width)) * 4
            } else {
                ((width * y) + x + (px % 8) + ((px / 8) * width)) * 4
            };
                
            match p {
                0 => {
                    screen[pos] = 0xe2;
                    screen[pos + 1] = 0xf3;
                    screen[pos + 2] = 0xe4;
                    screen[pos + 3] = 255;
                }
                1 => {
                    screen[pos] = 0x94;
                    screen[pos + 1] = 0xe3;
                    screen[pos + 2] = 0x44;
                    screen[pos + 3] = 255;
                }
                2 => {
                    screen[pos] = 0x46;
                    screen[pos + 1] = 0x87;
                    screen[pos + 2] = 0x8f;
                    screen[pos + 3] = 255;
                }
                3 => {
                    screen[pos] = 0x33;
                    screen[pos + 1] = 0x2c;
                    screen[pos + 2] = 0x50;
                    screen[pos + 3] = 255;
                }
                _ => {}
            }
        }
    }


    fn draw_background(&mut self, mem: &mut Memory) {
        let lcdc = mem.read(0xFF40);
        let mode = lcdc | 0b10000 > 0;
        let map = lcdc | 0b1000 > 0;

        let scy = mem.read(0xFF42) as usize;
        let scx = mem.read(0xFF43) as usize;

        let by = scy + SCREEN_HEIGHT;
        let bx = scx + SCREEN_WIDTH;

        //console::log_1(&format!("Mode: {} Map: {}", mode, map).into());

        let addr = if map {0x9C00} else {0x9800};
        for i in 0..(32 * 32){
            let tlx = (i % 32) * 8;
            let tly = (i / 32) * 8;
            let brx = tlx + 8;
            let bry = tly + 8;

            if (bx < tlx || scx > brx || by < tly || scy > bry) {
                // console::log_1(&format!("Not inter").into());
                // console::log_1(&format!("i: {} x: {} y: {} tlx: {} tly: {} scx: {} scy: {}",i, tlx - scx, tly - scy, tlx, tly, scx, scy).into());
                continue
            }

            let mut t_id = mem.read((addr + i) as u16) as usize;
            
            if !mode && t_id < 128 {
                t_id = t_id + 256
            }
            
            // TODO handle negatives
            if tlx >= scx && tly >= scy {
                // console::log_1(&format!("i: {} x: {} y: {} tlx: {} tly: {} scx: {} scy: {}",i, tlx - scx, tly - scy, tlx, tly, scx, scy).into());
                //self.paint_tile(t_id, tlx - scx, tly - scy, false);
            }
            
        }

    }

    pub fn draw(&mut self, mem: &mut Memory, ctx: &CanvasRenderingContext2d) {
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

    pub fn print(&self, id: usize) {
        if id == 0 {
            return
        }
            let tile = self;
            // println!("Tile {:#x} {:?}", id, tile);
            console::log_1(&format!("Tile {:#x} {:?}", id, tile).into());
            let mut s = String::new();
            for i in 0..8 {
                for j in 0..8 {
                    let symbol = match tile.data[(i * 8) + j] {
                        0 => " ",
                        1 => "░",
                        2 => "▒",
                        3 => "▓",
                        _ => ""
                    };
                    s.push_str(symbol);
                    //print!("{} ", symbol);
                }
                //println!();
                s.push_str("\n");
            }
            console::log_1(&s.into());;
    }
}