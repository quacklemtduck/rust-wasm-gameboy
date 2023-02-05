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
                Z: 0,
                N: 0,
                H: 0,
                C: 0,
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
        self.flags.Z = 1;
        self.flags.N = 0;
        self.flags.H = 1;
        self.flags.C = 1;
        self.sp = 0xfffe;
        self.pc = 0x100;
    }

    // Checks if the H flag should be set when adding a and b
    fn h_test(a: u8, b: u8) -> bool {
        return (a & 0xf) + (b & 0xf) > 0xf
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
        self.flags.Z = if value == 0 { 1 } else { 0 };
        self.flags.N = 0;
        self.flags.H = if CPU::h_test(orig, 1) { 1 } else { 0 };
        self.set_register_8(reg, value);
    }

    pub fn run(&mut self, mem: &mut Memory){
        let instruction = mem.read(self.pc);
        self.pc += 1;
        unsafe {
            match instruction {
                0x00 => {} // NOP
                0x02 => { // LD (BC), A
                    unsafe {
                        mem.write(self.bc.full, self.a);
                    }
                }
                0x03 => { // INC BC
                    unsafe {
                        self.bc.full += 1;
                    }
                }
                0x04 => { // INC B
                    self.inc_register_8(&Register8::B);
                }
                0x05 => { // DEC B
                    self.bc.sub.high -= 1;
                }
                0x06 => { // LD B, d8
                    let d8 = mem.read(self.pc);
                    self.pc += 1;
                    self.bc.sub.high = d8;
                }
                0x07 => { // RLCA
                    self.a.rotate_left(1);
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
    Z: u8,
    N: u8,
    H: u8,
    C: u8
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
            assert_eq!(cpu.flags.H, 1);
        }
    }
}
