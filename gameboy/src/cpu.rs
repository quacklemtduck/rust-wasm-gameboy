use crate::memory::Memory;

pub struct CPU {
    a: u8,
    flags: Flags,
    bc: Register,
    de: Register,
    hl: Register,
    sp: u16,
    pc: u16
}

impl CPU {
    pub fn new() -> CPU {
        return CPU{
            a: 0,
            flags: Flags{
                z: 0,
                n: 0,
                h: 0,
                cy: 0,
            },
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            sp: 0,
            pc: 0
        }
    }

    pub fn simulate_bootloader(&mut self) {
        self.a = 0x01;
        self.bc.sub.high = 0;
        self.bc.sub.low = 0x13;
        self.de.sub.high = 0;
        self.de.sub.low = 0xd8;
        self.hl.sub.high = 0x01;
        self.hl.sub.low = 0x4d;
        self.flags.z = 1;
        self.flags.n = 0;
        self.flags.h = 1;
        self.flags.cy = 1;
        self.sp = 0xfffe;
        self.pc = 0x100;
    }

    // Checks if the h flag should be set when adding a and b
    fn h_test(a: u8, b: u8) -> bool {
        return (a & 0xf) + (b & 0xf) > 0xf
    }
    fn h_test_16(a: u16, b: u16) -> bool {
        return (a & 0xfff) + (b & 0xfff) > 0xfff;
    }

    fn h_test_sub(a: u8, b: u8) -> bool {
        return (a & 0xf).wrapping_sub(b & 0xf) > 0x7f
    }

    fn get_register_8(&self, reg: &Register8) -> u8 {
        unsafe {
            match reg {
                Register8::A => self.a,
                Register8::B => self.bc.sub.high,
                Register8::C => self.bc.sub.low,
                Register8::D => self.de.sub.high,
                Register8::E => self.de.sub.low,
                Register8::H => self.hl.sub.high,
                Register8::L => self.hl.sub.low
            }
        }
    }

    fn get_register_16(&self, reg: &Register16) -> u16 {
        unsafe {
            match reg {
                Register16::BC => self.bc.full,
                Register16::DE => self.de.full,
                Register16::HL => self.hl.full,
                Register16::SP => self.sp,
                Register16::PC => self.pc
            }
        }
    }

    fn set_register_8(&mut self, reg: &Register8, val: u8) {
        unsafe {
            match reg {
                Register8::A => self.a = val,
                Register8::B => self.bc.sub.high = val,
                Register8::C => self.bc.sub.low = val,
                Register8::D => self.de.sub.high = val,
                Register8::E => self.de.sub.low = val,
                Register8::H => self.hl.sub.high = val,
                Register8::L => self.hl.sub.low = val
            }
        }
    }

    fn set_register_16(&mut self, reg: &Register16, val: u16) {
        unsafe {
            match reg {
                Register16::BC => self.bc.full = val,
                Register16::DE => self.de.full = val,
                Register16::HL => self.hl.full = val,
                Register16::SP => self.sp = val,
                Register16::PC => self.pc = val
            }
        }
    }

    fn inc_register_8(&mut self, reg: &Register8){
        let orig = self.get_register_8(reg);
        let value = if orig == 0xff {
            0
        } else {
            orig + 1
        };
        self.flags.z = if value == 0 { 1 } else { 0 };
        self.flags.n = 0;
        self.flags.h = if CPU::h_test(orig, 1) { 1 } else { 0 };
        self.set_register_8(reg, value);
    }

    fn inc_register_16(&mut self, reg: &Register16) {
        let orig = self.get_register_16(reg);
        let value = if orig == 0xffff {
            0
        } else {
            orig + 1
        };
        self.set_register_16(reg, value);
    }

    fn dec_register_8(&mut self, reg: &Register8) {
        let orig = self.get_register_8(reg);
        let value = if orig == 0 {
            0xff
        } else {
            orig - 1
        };
        self.flags.z = if value == 0 { 1 } else { 0 };
        self.flags.n = 1;
        self.flags.h = if CPU::h_test_sub(orig, 1) {1} else {0};
        self.set_register_8(reg, value);
    }

    fn dec_register_16(&mut self, reg: &Register16) {
        let orig = self.get_register_16(reg);
        let value = if orig == 0 {
            0xffff
        } else {
            orig - 1
        };
        self.set_register_16(reg, value);
    }

    fn add_hl(&mut self, reg: &Register16) {
        let reg_val = self.get_register_16(reg);
        let hl = self.get_register_16(&Register16::HL);
        let (value, overflow) = reg_val.overflowing_add(hl);
        self.set_register_16(&Register16::HL, value);

        self.flags.n = 0;
        if overflow {self.flags.cy = 1} else { self.flags.cy = 0 }
        if CPU::h_test_16(reg_val, hl) {
            self.flags.h = 1;
        } else {
            self.flags.h = 0;
        }
    }

    pub fn run(&mut self, mem: &mut Memory){
        let instruction = mem.read(self.pc);
        self.pc += 1;
        unsafe {
            match instruction {
                0x00 => {} // NOP
                0x01 => { // LD BC, d16
                    let value = mem.read_16(self.pc);
                    self.pc += 2;
                    self.set_register_16(&Register16::BC, value);
                }
                0x02 => { // LD (BC), A
                    mem.write(self.get_register_16(&Register16::BC), self.get_register_8(&Register8::A));

                }
                0x03 => { // INC BC
                    self.inc_register_16(&Register16::BC)
                }
                0x04 => { // INC B
                    self.inc_register_8(&Register8::B);
                }
                0x05 => { // DEC B
                    self.dec_register_8(&Register8::B);
                }
                0x06 => { // LD B, d8
                    let d8 = mem.read(self.pc);
                    self.pc += 1;
                    self.set_register_8(&Register8::B, d8);
                }
                0x07 => { // RLCA
                    let value = self.get_register_8(&Register8::A).rotate_left(1);
                    self.set_register_8(&Register8::A, value);
                    self.flags.cy = value & 0x01;
                    self.flags.h = 0;
                    self.flags.z = 0;
                    self.flags.n = 0;
                }
                0x08 => { // LD (a16), SP
                    let a16 = mem.read_16(self.pc);
                    self.pc += 2;
                    mem.write_16(a16, self.sp);
                }
                0x09 => { // ADD HL, BC
                    self.add_hl(&Register16::BC);
                }
                0x0A => { // LD A, (BC)
                    let val = mem.read(self.get_register_16(&Register16::BC));
                    self.a = val;
                }
                0x0B => { // DEC BC
                    self.dec_register_16(&Register16::BC);
                }
                0x0C => { // INC C
                    self.inc_register_8(&Register8::C);
                }
                0x0D => { // DEC C
                    self.dec_register_8(&Register8::C);
                }
                0x0E => { // LD C, d8
                    let d8 = mem.read(self.pc);
                    self.pc += 1;
                    self.set_register_8(&Register8::C, d8);
                }
                0x0F => { // RRCA
                    let value = self.get_register_8(&Register8::A).rotate_right(1);
                    self.set_register_8(&Register8::A, value);
                    self.flags.cy = value >> 7;
                    self.flags.h = 0;
                    self.flags.z = 0;
                    self.flags.n = 0;
                }
                // TODO 0x10 STOP
                0x11 => { // LD DE, d16
                    let value = mem.read_16(self.pc);
                    self.pc += 2;
                    self.set_register_16(&Register16::DE, value);
                }
                0x12 => { // LD (DE), A
                    mem.write(self.get_register_16(&Register16::DE), self.get_register_8(&Register8::A));
                }
                0x13 => { // INC DE
                    self.inc_register_16(&Register16::DE);
                }
                0x14 => { // INC D
                    self.inc_register_8(&Register8::D);
                }
                0x15 => { // DEC D
                    self.dec_register_8(&Register8::D);
                }
                0x16 => { // LD D, d8
                    let d8 = mem.read(self.pc);
                    self.pc += 1;
                    self.set_register_8(&Register8::D, d8);
                }
                0x17 => { // RLA
                    let orig = self.get_register_8(&Register8::A);
                    let mut value = orig << 1;
                    value = value | self.flags.cy;
                    self.flags.cy = orig >> 7;
                    self.set_register_8(&Register8::A, value);
                    self.flags.h = 0;
                    self.flags.z = 0;
                    self.flags.n = 0;
                }
                0x18 => { // JR s8
                    let mut s8 = mem.read(self.pc);
                    self.pc += 1;

                    if s8 >> 7 == 1 {
                        s8 = (!s8) + 1;
                        self.pc -= s8 as u16;
                    } else {
                        self.pc += s8 as u16;
                    }
                }
                0x19 => { // ADD HL, DE
                    self.add_hl(&Register16::DE);
                }
                0x1A => { // LD A, (DE)
                    let val = mem.read(self.get_register_16(&Register16::DE));
                    self.a = val;
                }
                0x1B => { // DEC DE
                    self.dec_register_16(&Register16::DE);
                }
                0x1C => { // INC E
                    self.inc_register_8(&Register8::E);
                }
                0x1D => { // DEC E
                    self.dec_register_8(&Register8::E);
                }
                0x1E => { // LD E, d8
                    let d8 = mem.read(self.pc);
                    self.pc += 1;
                    self.set_register_8(&Register8::E, d8);
                }
                0x1F => { // RRA
                    let orig = self.get_register_8(&Register8::A);
                    let mut value = orig >> 1;
                    value = (value & 0b01111111) | (self.flags.cy << 7);
                    self.flags.cy = orig & 0x01;
                    self.set_register_8(&Register8::A, value);
                    self.flags.h = 0;
                    self.flags.z = 0;
                    self.flags.n = 0;
                }

                _ => {
                    println!("Unsupported instruction: 0x{:02x}", instruction);
                    panic!("Unsupported instruction: 0x{:02x}", instruction);
                }
            }
        }
    }
}

struct Flags {
    z: u8,
    n: u8,
    h: u8,
    cy: u8
}

enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}

enum Register16 {
    BC,
    DE,
    HL,
    SP,
    PC
}

#[repr(C)]
union Register {
    sub: SubRegister,
    full: u16
}

impl Register {
    fn new() -> Register {
        return Register{sub: SubRegister::new()}
    }
}

#[derive(Copy, Clone)]
struct SubRegister {
    low: u8,
    high: u8
}

impl SubRegister {
    fn new() -> SubRegister {
        return SubRegister{high: 0, low: 0}
    }
}

#[cfg(test)]
mod cpu_tests {
    use crate::cpu::{CPU, Register};
    use crate::memory::Memory;

    #[test]
    fn register_union_works() {
        let mut reg = Register::new();
        reg.sub.low = 8;
        unsafe {
            assert_eq!(8, reg.full);
        }
        reg.full = 0x0100;
        unsafe {
            assert_eq!(0, reg.sub.low);
            assert_eq!(1, reg.sub.high);
        }
    }

    #[test]
    fn cpu_noop() {
        let mut mem = Memory::new();
        let mut cpu = CPU::new();
        cpu.run(&mut mem);
    }

    #[test]
    fn cpu_inc_b() {
        unsafe {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.bc.sub.high = 0xff;
            mem.write(0, 0x04);
            cpu.run(&mut mem);

            assert_eq!(cpu.bc.sub.high, 0);
            assert_eq!(cpu.flags.h, 1);
        }
    }

    #[test]
    fn cpu_rlca() {
        unsafe {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.a = 0x01 << 7;
            mem.write(0, 0x07);
            cpu.run(&mut mem);

            assert_eq!(cpu.flags.cy, 1);
            assert_eq!(cpu.a, 0x01);
        }
    }

    #[test]
    fn cpu_rrca() {
        unsafe {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.a = 0x01;
            mem.write(0, 0x0F);
            cpu.run(&mut mem);

            assert_eq!(cpu.flags.cy, 1);
            assert_eq!(cpu.a, 0x01 << 7);
        }
    }

    #[test]
    fn cpu_rla() {
        unsafe {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.a = 0x01 << 7;
            mem.write(0, 0x17);
            cpu.run(&mut mem);

            assert_eq!(cpu.flags.cy, 1);
            assert_eq!(cpu.a, 0x00);
        }
    }

    #[test]
    fn cpu_rra() {
        unsafe {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.a = 0x01;
            mem.write(0, 0x1F);
            cpu.run(&mut mem);

            assert_eq!(cpu.flags.cy, 1);
            assert_eq!(cpu.a, 0x00);
        }
    }

    #[test]
    fn cpu_jr() {
        unsafe {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            mem.write(0, 0x18);
            mem.write(1, 0b11111110);
            cpu.run(&mut mem);

            assert_eq!(cpu.pc, 0);

            mem.write(1, 0x01);
            cpu.run(&mut mem);

            assert_eq!(cpu.pc, 3);
        }
    }
}
