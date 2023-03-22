use crate::memory::Memory;

pub struct PPU {
    tile_map: [Tile; 384]
}

impl PPU {
    pub fn new() -> PPU {
        PPU{tile_map: [Tile::new(); 384]}
    }

    pub fn advance_line(&self, mem: &mut Memory) {
        println!("Advance");
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