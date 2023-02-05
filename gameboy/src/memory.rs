pub struct Memory {
    pub mem: [u8; 0xFFFF]
}

impl Memory {
    pub fn new() -> Memory {
        return Memory{mem: [0; 0xFFFF] }
    }

    pub fn read(&self, loc: u16) -> u8{
        return self.mem[loc as usize]
    }

    pub fn read_16(&self, loc: u16) -> u16 {
        let low = self.read(loc);
        let high = self.read(loc + 1);
        return ((high as u16) << 8) | low as u16;
    }

    pub fn write(&mut self, loc: u16, val: u8){
        self.mem[loc as usize] = val
    }

    pub fn write_16(&mut self, loc: u16, val: u16) {
        let low = (val & 0xff) as u8;
        let high = (val >> 8) as u8;
        self.write(loc, low);
        self.write(loc + 1, high);
    }

    pub fn simulate_bootloader(&mut self) {
        self.write(0xff05, 0x00);
        self.write(0xff06, 0x00);
        self.write(0xff07, 0x00);
        self.write(0xff10, 0x80);
        self.write(0xff11, 0x80);
        self.write(0xff12, 0xf3);
        self.write(0xff13, 0xc1);
        self.write(0xff14, 0x87);
        self.write(0xff16, 0x3f);
        self.write(0xff17, 0x00);
        self.write(0xff19, 0xbf);
        self.write(0xff1a, 0x7f);
        self.write(0xff1b, 0xff);
        self.write(0xff1c, 0x9f);
        self.write(0xff1e, 0xbf);
        self.write(0xff20, 0xff);
        self.write(0xff21, 0x00);
        self.write(0xff22, 0x00);
        self.write(0xff23, 0xbf);
        self.write(0xff24, 0x77);
        self.write(0xff25, 0xf3);
        self.write(0xff26, 0x80);
        self.write(0xff40, 0x91);
        self.write(0xff42, 0x00);
        self.write(0xff43, 0x00);
        self.write(0xff44, 0x8f);
        self.write(0xff45, 0x00);
        self.write(0xff47, 0xfc);
        self.write(0xff48, 0xff);
        self.write(0xff49, 0xff);
        self.write(0xff4a, 0x00);
        self.write(0xff4b, 0x00);
        self.write(0xff50, 0x01);
        self.write(0xfffb, 0x01);
        self.write(0xfffc, 0x2e);
        self.write(0xfffd, 0x00);
        self.write(0xffff, 0x00);
    }
}