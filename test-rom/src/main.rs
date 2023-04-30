use std::fs;

const NOP: u8 = 0x00;
const LD_BC_D16: u8 = 0x01;
const LD_DE_D16: u8 = 0x11;
const LD_HL_D16: u8 = 0x21;
const LD_A_DE: u8 = 0x1A;
const INC_DE: u8 = 0x13;
const LD_HL_INC_A: u8 = 0x22;
const DEC_BC: u8 = 0x0B;
const DEC_C: u8 = 0x0D;
const LD_A_B: u8 = 0x78;
const OR_C: u8 = 0xB1;
const JR_NZ: u8 = 0x20;
const LD_A_D8: u8 = 0x3e;
const LD_A8_A: u8 = 0xE0;
const LD_A_A8: u8 = 0xF0;
const JR: u8 = 0x18;
const JP: u8 = 0xc3;
const EI: u8 = 0xFB;
const DI: u8 = 0xF3;
const HALT: u8 = 0x76;
const RET: u8 = 0xC9;
const RETI: u8 = 0xD9;
const LD_A16_A: u8 = 0xEA;
const PUSH_HL: u8 = 0xE5;
const POP_HL: u8 = 0xE1;
const INC_H_HL: u8 = 0x34;



struct Program {
    pc: usize,
    rom: [u8; 0x8000],
}

impl Program {
    fn new() -> Program {
        Program { pc: 0x100, rom: [0; 0x8000] }
    }

    fn write(&mut self, value: u8) {
        self.rom[self.pc] = value;
        self.pc += 1;
    }

    fn write_signed(&mut self, value: i8) {
        self.rom[self.pc] = value as u8;
        self.pc += 1;
    }

    fn write_16(&mut self, value: u16) {
        set_u16(&mut self.rom, self.pc, value);
        self.pc += 2;
    }

    fn write_multiple(&mut self, values: &[u8]) {
        set_multiple(&mut self.rom, self.pc, values);
        self.pc += values.len();
    }

    fn set_pc(&mut self, value: usize) {
        self.pc = value;
    }

    fn save_file(&self) {
        fs::write("test_rom.gb", &self.rom).expect("Unable to write file");
    }
}

fn main() {
    let mut p = Program::new();

    // Nintendo Logo
    p.set_pc(0x0104);
    p.write_multiple(&[0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E]);

    //Title
    p.set_pc(0x0134);
    p.write_multiple(&[65, 32, 84, 69, 83, 84, 32, 82, 79, 77 ]);

    // ROM only
    p.set_pc(0x0147);
    p.write(0x00);
    p.write(0x00);

    // Actual program

    // Graphics
    let tiles = 
    [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x3C,0x3C,0x66,0x66,0x66,0x66,0x7E,0x7E,
    0x66,0x66,0x66,0x66,0x66,0x66,0x00,0x00,
    0x06,0x06,0x06,0x06,0x06,0x06,0x06,0x06,
    0x06,0x06,0x66,0x66,0x3C,0x3C,0x00,0x00,
    0xC6,0xC6,0xEE,0xEE,0xFE,0xFE,0xD6,0xD6,
    0xC6,0xC6,0xC6,0xC6,0xC6,0xC6,0x00,0x00,
    0x7C,0x7C,0x66,0x66,0x66,0x66,0x7C,0x7C,
    0x60,0x60,0x60,0x60,0x60,0x60,0x00,0x00,
    0x00,0x00,0x00,0x00,0x3C,0x3C,0x06,0x06,
    0x3E,0x3E,0x66,0x66,0x3E,0x3E,0x00,0x00,
    0x60,0x60,0x60,0x60,0x7C,0x7C,0x66,0x66,
    0x66,0x66,0x66,0x66,0x7C,0x7C,0x00,0x00,
    0x06,0x06,0x06,0x06,0x3E,0x3E,0x66,0x66,
    0x66,0x66,0x66,0x66,0x3E,0x3E,0x00,0x00,
    0x00,0x00,0x00,0x00,0x3C,0x3C,0x66,0x66,
    0x7E,0x7E,0x60,0x60,0x3C,0x3C,0x00,0x00,
    0x00,0x00,0x00,0x00,0x7C,0x7C,0x66,0x66,
    0x66,0x66,0x66,0x66,0x66,0x66,0x00,0x00,
    0x00,0x00,0x00,0x00,0x7C,0x7C,0x66,0x66,
    0x60,0x60,0x60,0x60,0x60,0x60,0x00,0x00,
    0x00,0x00,0x00,0x00,0x3C,0x3C,0x60,0x60,
    0x3C,0x3C,0x06,0x06,0x7C,0x7C,0x00,0x00,
    0x30,0x30,0x30,0x30,0x7C,0x7C,0x30,0x30,
    0x30,0x30,0x30,0x30,0x1C,0x1C,0x00,0x00,
    0x00,0x00,0x00,0x00,0x66,0x66,0x66,0x66,
    0x66,0x66,0x66,0x66,0x3E,0x3E,0x00,0x00,
    0x00,0x00,0x00,0x00,0x66,0x66,0x66,0x66,
    0x66,0x66,0x3C,0x3C,0x18,0x18,0x30,0x30,
    0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,
    0x18,0x18,0x00,0x00,0x18,0x18,0x00,0x00,
    0x00,0x00,0x00,0x00,0x04,0x04,0x06,0x06,
    0x05,0x07,0x0D,0x0F,0x0F,0x08,0x8F,0x6F,
    0x7E,0x2D,0xBD,0x2E,0xAF,0x4F,0x48,0x0F,
    0x07,0x07,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0xF8,0xF8,0xFC,0x04,0xE2,0x1E,
    0xFF,0x81,0xFF,0x81,0xFA,0x06,0x04,0xFC,
    0xF8,0xF8,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x04,0x04,0x06,0x06,
    0x05,0x07,0x0D,0x0F,0x0F,0x08,0x4F,0x2F,
    0x7E,0x2D,0x9D,0x6E,0x4F,0x2F,0x28,0x0F,
    0x07,0x07,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x6C,0x6C,0x92,0x92,0x82,0x82,
    0x82,0x82,0x44,0x44,0x28,0x28,0x10,0x10,
    0x10,0x10,0x18,0x18,0xFF,0xFF,0x7E,0x7E,
    0x3C,0x3C,0x7E,0x7E,0x66,0x66,0x00,0x00];

    p.set_pc(0x4000);
    p.write_multiple(&tiles);

    let map = [
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x17,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x17,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x17,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x17,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x17,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x17,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x03,0x05,0x07,0x08,0x00,
  0x06,0x0E,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x17,0x00,0x00,0x01,0x09,0x07,0x0A,0x08,0x05,
  0x0B,0x00,0x02,0x0D,0x0B,0x0C,0x00,0x00,0x00,0x00,
  0x00,0x00,0x17,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x04,0x08,
  0x0C,0x08,0x0A,0x0B,0x08,0x09,0x00,0x16,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x17,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x17,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x17,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x17,0x00,
  0x00,0x00,0x17,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x17,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x17,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x17,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x17,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x17,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x17,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x17,0x00,0x00,0x00,0x17,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x17,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x17,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  0x00,0x00,0x00,0x00];
    //p.set_pc(0x9800 + (4 * 32) + 7);
    p.set_pc(0x5000);
    p.write_multiple(&map);

    let sprites = [
        100, 0, 0x10, 0,
        100, 8, 0x12, 0,
    ];
    p.set_pc(0x6000);
    p.write_multiple(&sprites);

    // VBLANK handler
    p.set_pc(0x40);
    p.write(PUSH_HL);
    // Move sprite 1
    p.write(LD_HL_D16);
    p.write_16(0xFE01);
    p.write(INC_H_HL);
    // Move sprite 2
    p.write(LD_HL_D16);
    p.write_16(0xFE05);
    p.write(INC_H_HL);

    p.write(POP_HL);
    p.write(RETI);

    p.set_pc(0x100);
    p.write(NOP);
    p.write(JP);
    p.write_16(0x0150);
    p.set_pc(0x150);

    // Enable VBLANK
    p.write(LD_A_D8);
    p.write(0x01);
    p.write(LD_A8_A);
    p.write(0xFF);
    p.write(EI);

    // Wait for VBLANK and then turn off PPU to load graphics
    p.write(HALT);
    p.write(NOP);
    p.write(LD_A_D8);
    p.write(0);
    p.write(LD_A8_A);
    p.write(0x40);
    p.write(DI);

    // Clear screen
    p.write(LD_HL_D16);
    p.write_16(0x9800);
    p.write(LD_BC_D16);
    p.write_16(0x400);
    let mut pc = p.pc as i32;
    p.write(LD_A_D8);
    p.write(0);
    p.write(LD_HL_INC_A);
    p.write(DEC_BC);
    p.write(LD_A_B);
    p.write(OR_C);
    p.write(JR_NZ);
    p.write_signed((pc - 1 - (p.pc as i32)) as i8);

    // Set palletes
    p.write(LD_A_D8);
    p.write(0b00011011);
    p.write(LD_A8_A);
    p.write(0x47);
    p.write(LD_A_D8);
    p.write(0b11100000);
    p.write(LD_A8_A);
    p.write(0x48);
    p.write(LD_A8_A);
    p.write(0x49);

    // Load tiles
    // DE = From
    // HL = To
    p.write(LD_DE_D16);
    p.write_16(0x4000);

    p.write(LD_HL_D16);
    p.write_16(0x8000);

    p.write(LD_BC_D16);
    p.write_16(tiles.len() as u16);

    // Load byte
    pc = p.pc as i32;
    //p.write(HALT);
    p.write(LD_A_DE);
    p.write(INC_DE);
    p.write(LD_HL_INC_A);

    // Has all been loaded?
    p.write(DEC_BC);
    p.write(LD_A_B);
    p.write(OR_C);
    p.write(JR_NZ);
    p.write_signed((pc - 1 - (p.pc as i32)) as i8);

    // Load Map
    p.write(LD_DE_D16);
    p.write_16(0x5000);

    p.write(LD_HL_D16);
    p.write_16(0x9800);

    p.write(LD_BC_D16);
    p.write_16(map.len() as u16);

    // Load byte
    pc = p.pc as i32;
    //p.write(HALT);
    p.write(LD_A_DE);
    p.write(INC_DE);
    p.write(LD_HL_INC_A);

    // Has all been loaded?
    p.write(DEC_BC);
    p.write(LD_A_B);
    p.write(OR_C);
    p.write(JR_NZ);
    p.write_signed((pc - 1 - (p.pc as i32)) as i8);

    // Clear Sprites
    p.write(LD_HL_D16);
    p.write_16(0xFE00);
    p.write(LD_BC_D16);
    p.write_16(0x9F);
    pc = p.pc as i32;
    p.write(LD_A_D8);
    p.write(0);
    p.write(LD_HL_INC_A);
    p.write(DEC_BC);
    p.write(LD_A_B);
    p.write(OR_C);
    p.write(JR_NZ);
    p.write_signed((pc - 1 - (p.pc as i32)) as i8);

    // Load sprites
    p.write(LD_A_D8);
    p.write(100); // Y
    p.write(LD_A16_A);
    p.write_16(0xFE00);
    p.write(LD_A16_A);
    p.write_16(0xFE04);

    // X1
    p.write(LD_A_D8);
    p.write(0); 
    p.write(LD_A16_A);
    p.write_16(0xFE01);

    // X2
    p.write(LD_A_D8);
    p.write(8); 
    p.write(LD_A16_A);
    p.write_16(0xFE05);

    // Tiles
    p.write(LD_A_D8);
    p.write(0x10); 
    p.write(LD_A16_A);
    p.write_16(0xFE02);

    p.write(LD_A_D8);
    p.write(0x12); 
    p.write(LD_A16_A);
    p.write_16(0xFE06);

    // Controls 
    p.write(LD_A_D8);
    p.write(0b10010111);
    p.write(LD_A8_A);
    p.write(0x40);

    p.write(EI);
    p.write(NOP);
    p.write(HALT);
    p.write(NOP);
    p.write(JR);
    p.write_signed(-4);

    p.save_file();
}

fn set_multiple(rom: &mut [u8], position: usize, values: &[u8]) {
    rom[position..(position + values.len())].copy_from_slice(values)
}

fn set_u16(rom: &mut [u8], position: usize, value: u16) {
    rom[position] = (value & 0xFF) as u8;
    rom[position + 1] = (value >> 8) as u8;
}

