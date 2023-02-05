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

    pub fn write(&mut self, loc: u16, val: u8){
        self.mem[loc as usize] = val
    }
}