use crate::memory::Memory;

pub struct CPU {
    a: u8,
    flags: Flags,
    bc: Register,
    de: Register,
    hl: Register,
    sp: u16,
    pc: u16,
    ime: bool
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
                lower: 0
            },
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            sp: 0,
            pc: 0,
            ime: false
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

    fn set_register_16(&mut self, reg: &Register16, val: u16) {
        match reg {
            Register16::BC => self.bc.full = val,
            Register16::DE => self.de.full = val,
            Register16::HL => self.hl.full = val,
            Register16::SP => self.sp = val,
            Register16::PC => self.pc = val
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

    fn inc_mem(&mut self, mem: &mut Memory, addr: u16) {
        let orig = mem.read(addr);
        let value = if orig == 0xff {
            0
        } else {
            orig + 1
        };
        self.flags.z = if value == 0 { 1 } else { 0 };
        self.flags.n = 0;
        self.flags.h = if CPU::h_test(orig, 1) { 1 } else { 0 };
        mem.write(addr, value);
    }

    fn dec_mem(&mut self, mem: &mut Memory, addr: u16) {
        let orig = mem.read(addr);
        let value = if orig == 0 {
            0xff
        } else {
            orig - 1
        };
        self.flags.z = if value == 0 { 1 } else { 0 };
        self.flags.n = 1;
        self.flags.h = if CPU::h_test_sub(orig, 1) {1} else {0};
        mem.write(addr, value);
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

    fn jump_relative(&mut self, s8: u8){
        let mut s8 = s8;
        if s8 >> 7 == 1 {
            s8 = (!s8) + 1;
            self.pc -= s8 as u16;
        } else {
            self.pc += s8 as u16;
        }
    }

    fn ld_r_r(&mut self, reg1: &Register8, reg2: &Register8) {
        let val = self.get_register_8(reg2);
        self.set_register_8(reg1, val);
    }

    fn ld_r_hl(&mut self, mem: &Memory, reg: &Register8) {
        let val = mem.read(self.get_register_16(&Register16::HL));
        self.set_register_8(reg, val);
    }

    fn ld_hl_r(&self, mem: &mut Memory, reg: &Register8) {
        let val = self.get_register_8(reg);
        let addr = self.get_register_16(&Register16::HL);
        mem.write(addr, val);
    }

    fn add_to_a(&mut self, val: u8){
        let orig = self.get_register_8(&Register8::A);
        let (value, carry) = orig.overflowing_add(val);
        self.set_register_8(&Register8::A, value);
        if CPU::h_test(orig, val) {
            self.flags.h = 1;
        } else {
            self.flags.h = 0;
        }
        self.flags.n = 0;
        if carry {
            self.flags.cy = 1;
        } else {
            self.flags.cy = 0;
        }
        if value == 0 {
            self.flags.z = 1;
        } else {
            self.flags.z = 0;
        }
    }

    fn sub_from_a(&mut self, val: u8){
        let orig = self.get_register_8(&Register8::A);
        let (value, carry) = orig.overflowing_sub(val);
        self.set_register_8(&Register8::A, value);
        if CPU::h_test_sub(orig, val) {
            self.flags.h = 1;
        } else {
            self.flags.h = 0;
        }
        self.flags.n = 1;
        if carry {
            self.flags.cy = 1;
        } else {
            self.flags.cy = 0;
        }
        if value == 0 {
            self.flags.z = 1;
        } else {
            self.flags.z = 0;
        }
    }

    fn adc_to_a(&mut self, val: u8){
        let orig = self.get_register_8(&Register8::A);
        let cy = self.flags.cy;
        let (mut value, mut carry) = orig.overflowing_add(val);
        let (value2, carry2) = value.overflowing_add(cy);
        value = value2;
        carry  = carry || carry2;
        self.set_register_8(&Register8::A, value);
        if (orig & 0xf) + (val &0xf) + cy > 0xf {
            self.flags.h = 1;
        } else {
            self.flags.h = 0; // TODO In case of bugs, remove?
        }
        self.flags.n = 0;
        if carry {
            self.flags.cy = 1;
        } else {
            self.flags.cy = 0;
        }
        if value == 0 {
            self.flags.z = 1;
        } else {
            self.flags.z = 0;
        }
    }

    fn sbc_from_a(&mut self, val: u8){
        let orig = self.get_register_8(&Register8::A);
        let cy = self.flags.cy;
        let (mut value, mut carry) = orig.overflowing_sub(val);
        let (value2, carry2) = value.overflowing_sub(cy);
        value = value2;
        carry  = carry || carry2;
        self.set_register_8(&Register8::A, value);
        if (orig & 0xf).wrapping_sub(val & 0xf).wrapping_sub(cy) > 0x7f {
            self.flags.h = 1;
        } else {
            self.flags.h = 0; // TODO In case of bugs, remove?
        }
        self.flags.n = 1;
        if carry {
            self.flags.cy = 1;
        } else {
            self.flags.cy = 0;
        }
        if value == 0 {
            self.flags.z = 1;
        } else {
            self.flags.z = 0;
        }
    }

    fn and_a(&mut self, val: u8){
        let orig = self.get_register_8(&Register8::A);
        let value = orig & val;
        self.set_register_8(&Register8::A, value);
        self.flags.h = 1;
        self.flags.cy = 0;
        self.flags.n = 0;
        if value == 0 {
            self.flags.z = 1;
        }else{
            self.flags.z = 0;
        }
    }

    fn xor_a(&mut self, val: u8){
        let orig = self.get_register_8(&Register8::A);
        let value = orig ^ val;
        self.set_register_8(&Register8::A, value);
        self.flags.h = 0;
        self.flags.cy = 0;
        self.flags.n = 0;
        if value == 0 {
            self.flags.z = 1;
        }else{
            self.flags.z = 0;
        }
    }

    fn or_a(&mut self, val: u8){
        let orig = self.get_register_8(&Register8::A);
        let value = orig | val;
        self.set_register_8(&Register8::A, value);
        self.flags.h = 0;
        self.flags.cy = 0;
        self.flags.n = 0;
        if value == 0 {
            self.flags.z = 1;
        }else{
            self.flags.z = 0;
        }
    }

    fn cp(&mut self, a: u8, b: u8){
        let result = a - b;
        if result == 0 {
            self.flags.z = 1;
        } else {
            self.flags.z = 0;
        }
        self.flags.n = 1;
        if (a & 0xF) < (b & 0xF) {
            self.flags.h = 1;
        } else {
            self.flags.h = 0;
        }
        if a < b {
            self.flags.cy = 1;
        } else {
            self.flags.cy = 0;
        }
    }

    fn pop(&mut self, mem: &mut Memory) -> u16{
        let addr = self.get_register_16(&Register16::SP);
        let value = mem.read_16(addr);
        self.set_register_16(&Register16::SP, addr + 2);
        return value
    }

    fn push(&mut self, mem: &mut Memory, value: u16) {
        let addr = self.get_register_16(&Register16::SP) - 2;
        mem.write_16(addr, value);
        self.set_register_16(&Register16::SP, addr);
    }

    fn set_interrupt(&mut self, value: bool){
        self.ime = value;
    }

    fn rlc(&mut self, reg: &Register8, check_z: bool) {
        let value = self.get_register_8(reg).rotate_left(1);
        self.set_register_8(reg, value);
        self.flags.cy = value & 0x01;
        self.flags.h = 0;
        if check_z {
            self.flags.z = if value == 0 {1} else {0};
        } else {
            self.flags.z = 0;
        }
        self.flags.n = 0;
    }

    fn rl(&mut self, reg: &Register8, check_z: bool) {
        let orig = self.get_register_8(reg);
        let mut value = orig << 1;
        value = value | self.flags.cy;
        self.flags.cy = orig >> 7;
        self.set_register_8(reg, value);
        self.flags.h = 0;
        if check_z {
            self.flags.z = if value == 0 {1} else {0};
        } else {
            self.flags.z = 0;
        }
        self.flags.n = 0;
    }

    fn rrc(&mut self, reg: &Register8, check_z: bool){
        let value = self.get_register_8(reg).rotate_right(1);
        self.set_register_8(reg, value);
        self.flags.cy = (value>>7) & 0x01;
        self.flags.h = 0;
        if check_z {
            self.flags.z = if value == 0 {1} else {0};
        } else {
            self.flags.z = 0;
        }
        self.flags.n = 0;
    }

    fn rr(&mut self, reg: &Register8, check_z: bool){
        let orig = self.get_register_8(reg);
        let mut value = orig >> 1;
        value = (value & 0b01111111) | (self.flags.cy << 7);
        self.flags.cy = orig & 0x01;
        self.set_register_8(reg, value);
        self.flags.h = 0;
        if check_z {
            self.flags.z = if value == 0 {1} else {0};
        } else {
            self.flags.z = 0;
        }
        self.flags.n = 0;
    }

    fn sl(&mut self, reg: &Register8) {
        let val = self.get_register_8(reg) << 1;
        self.set_register_8(reg, val);
        self.flags.cy = val >> 7;
        self.flags.z = if val == 0 {1} else {0};
        self.flags.n = 0;
        self.flags.h = 0;
    }

    fn sra(&mut self, reg: &Register8) {
        let orig = self.get_register_8(reg);
        let val = (orig >> 1) | (orig & 0x80);
        self.set_register_8(reg, val);
        self.flags.cy = val & 0x01;
        self.flags.z = if val == 0 {1} else {0};
        self.flags.n = 0;
        self.flags.h = 0;
    }

    fn srl(&mut self, reg: &Register8) {
        let orig = self.get_register_8(reg);
        let val = orig >> 1;
        self.set_register_8(reg, val);
        self.flags.cy = val & 0x01;
        self.flags.z = if val == 0 {1} else {0};
        self.flags.n = 0;
        self.flags.h = 0;
    }

    fn bit(&mut self, reg: &Register8, bit: u8) {
        let val = self.get_register_8(reg);
        self.flags.z = if (val & (1 << bit)) != 0 {1} else {0};
        self.flags.n = 0;
        self.flags.h = 1;
    }

    fn res(&mut self, reg: &Register8, bit: u8) {
        let orig = self.get_register_8(reg);
        let val = orig & !(1 << bit);
        self.set_register_8(reg, val);
    }

    fn set(&mut self, reg: &Register8, bit: u8) {
        let orig = self.get_register_8(reg);
        let val = orig | (1 << bit);
        self.set_register_8(reg, val);
    }

    fn swap(&mut self, reg: &Register8) {
        let orig = self.get_register_8(reg);
        let value = (orig << 4) | (orig >> 4);
        self.set_register_8(reg, value);
        self.flags.z = if value == 0 {1} else {0};
        self.flags.cy = 0;
        self.flags.n = 0;
        self.flags.h = 0;
    }

    pub fn run(&mut self, mem: &mut Memory){
        let instruction = mem.read(self.pc);
        self.pc += 1;
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
                self.rlc(&Register8::A, false);
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
                self.rrc(&Register8::A, false);
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
                self.rl(&Register8::A, false);
            }
            0x18 => { // JR s8
                let s8 = mem.read(self.pc);
                self.pc += 1;
                self.jump_relative(s8);
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
                self.rr(&Register8::A, false);
            }
            0x20 => { // JR NZ, s8
                let s8 = mem.read(self.pc);
                self.pc += 1;
                if self.flags.z == 0 {
                    self.jump_relative(s8)
                }
            }
            0x21 => { // LD HL, d16
                let d16 = mem.read_16(self.pc);
                self.pc += 2;
                self.set_register_16(&Register16::HL, d16);
            }
            0x22 => { // LD (HL+), A
                mem.write(self.get_register_16(&Register16::HL), self.get_register_8(&Register8::A));
                self.inc_register_16(&Register16::HL);
            }
            0x23 => { // INC HL
                self.inc_register_16(&Register16::HL);
            }
            0x24 => { // INC H
                self.inc_register_8(&Register8::H);
            }
            0x25 => { // DEC H
                self.dec_register_8(&Register8::H);
            }
            0x26 => { // LD H, d8
                let d8 = mem.read(self.pc);
                self.pc += 1;
                self.set_register_8(&Register8::H, d8);
            }
            0x27 => { // DAA
                let orig =self.get_register_8(&Register8::A);
                let mut value = orig;
                let mut carry = false;
                if self.flags.h == 0 {
                    let low = orig & 0x0f;
                    if low > 9 || (self.flags.h > 0){
                        let (v, c) = value.overflowing_add(6);
                        value = v;
                        carry = carry || c;
                    }
                    if value > 0x9f || self.flags.cy > 0 {
                        let (v, c) = value.overflowing_add(0x60);
                        value = v;
                        carry = carry || c;
                    }
                } else {
                    if self.flags.h > 0 {
                        let (v, _c) = value.overflowing_sub(6);
                        value = v;
                    }
                    if self.flags.cy > 0 {
                        let (v, c) = value.overflowing_sub(0x60);
                        value = v;
                        carry = carry || c;
                    }
                }
                self.set_register_8(&Register8::A, value);
                self.flags.h = 0;
                if value == 0 {
                    self.flags.z = 1;
                } else {
                    self.flags.z = 0;
                }
                if carry {
                    self.flags.cy = 1;
                } else {
                    self.flags.cy = 0;
                }
            }
            0x28 => { // JR Z, s8
                let s8 = mem.read(self.pc);
                self.pc += 1;
                if self.flags.z == 1 {
                    self.jump_relative(s8);
                }
            }
            0x29 => { // ADD HL, HL
                self.add_hl(&Register16::HL);
            }
            0x2A => { // LD A, (HL+)
                let val = mem.read(self.get_register_16(&Register16::DE));
                self.a = val;
                self.inc_register_16(&Register16::HL);
            }
            0x2B => { // DEC HL
                self.dec_register_16(&Register16::HL);
            }
            0x2C => { // INC L
                self.inc_register_8(&Register8::L);
            }
            0x2D => { // DEC L
                self.dec_register_8(&Register8::L);
            }
            0x2E => { // LD L, d8
                let d8 = mem.read(self.pc);
                self.pc += 1;
                self.set_register_8(&Register8::L, d8);
            }
            0x2F => { // CPL inverts A
                let value = !self.get_register_8(&Register8::A);
                self.set_register_8(&Register8::A, value);
                self.flags.n = 1;
                self.flags.h = 1;
            }
            0x30 => { // JR NC, s8
                let s8 = mem.read(self.pc);
                self.pc += 1;
                if self.flags.cy == 0 {
                    self.jump_relative(s8)
                }
            }
            0x31 => { // LD SP, d16
                let d16 = mem.read_16(self.pc);
                self.pc += 2;
                self.set_register_16(&Register16::SP, d16);
            }
            0x32 => { // LD (HL-), A
                mem.write(self.get_register_16(&Register16::HL), self.get_register_8(&Register8::A));
                self.dec_register_16(&Register16::HL);
            }
            0x33 => { // INC SP
                self.inc_register_16(&Register16::SP);
            }
            0x34 => { // INC (HL)
                let addr = self.get_register_16(&Register16::HL);
                self.inc_mem(mem, addr);
            }
            0x35 => { // DEC (HL)
                let addr = self.get_register_16(&Register16::HL);
                self.dec_mem(mem, addr);
            }
            0x36 => { // LD (HL), d8
                let d8 = mem.read(self.pc);
                self.pc += 1;
                let addr = self.get_register_16(&Register16::HL);
                mem.write(addr, d8);
            }
            0x37 => { // SCF
                self.flags.cy = 1;
                self.flags.h = 0;
                self.flags.n = 0;
            }
            0x38 => { // JR C, s8
                let s8 = mem.read(self.pc);
                self.pc += 1;
                if self.flags.cy == 1 {
                    self.jump_relative(s8);
                }
            }
            0x39 => { // ADD, HL, SP
                self.add_hl(&Register16::SP);
            }
            0x3A => { // LD A, (HL-)
                let val = mem.read(self.get_register_16(&Register16::DE));
                self.a = val;
                self.dec_register_16(&Register16::HL);
            }
            0x3B => { // DEC SP
                self.dec_register_16(&Register16::SP);
            }
            0x3C => { // INC A
                self.inc_register_8(&Register8::A);
            }
            0x3D => { // DEC A
                self.dec_register_8(&Register8::A);
            }
            0x3E => { // LD A, d8
                let d8 = mem.read(self.pc);
                self.pc += 1;
                self.set_register_8(&Register8::A, d8);
            }
            0x3F => { // CCF
                if self.flags.cy > 0 {
                    self.flags.cy = 0;
                } else {
                    self.flags.cy = 1;
                }
                self.flags.n = 0;
                self.flags.h = 0;
            }
            0x40 => { // LD B, B
                self.ld_r_r(&Register8::B, &Register8::B);
            }
            0x41 => { // LD B, C
                self.ld_r_r(&Register8::B, &Register8::C);
            }
            0x42 => { // LD B, D
                self.ld_r_r(&Register8::B, &Register8::D);
            }
            0x43 => { // LD B, E
                self.ld_r_r(&Register8::B, &Register8::E);
            }
            0x44 => { // LD B, H
                self.ld_r_r(&Register8::B, &Register8::H);
            }
            0x45 => { // LD B, L
                self.ld_r_r(&Register8::B, &Register8::L);
            }
            0x46 => { // LD B, (HL)
                self.ld_r_hl(mem, &Register8::B);
            }
            0x47 => { // LD B, A
                self.ld_r_r(&Register8::B, &Register8::A);
            }
            0x48 => { // LD C, B
                self.ld_r_r(&Register8::C, &Register8::B);
            }
            0x49 => { // LD C, C
                self.ld_r_r(&Register8::C, &Register8::C);
            }
            0x4A => { // LD C, D
                self.ld_r_r(&Register8::C, &Register8::D);
            }
            0x4B => { // LD C, E
                self.ld_r_r(&Register8::C, &Register8::E);
            }
            0x4C => { // LD C, H
                self.ld_r_r(&Register8::C, &Register8::H);
            }
            0x4D => { // LD C, L
                self.ld_r_r(&Register8::C, &Register8::L);
            }
            0x4E => { // LD C, (HL)
                self.ld_r_hl(mem, &Register8::C);
            }
            0x4F => { // LD C, A
                self.ld_r_r(&Register8::C, &Register8::A);
            }
            0x50 => { // LD D, B
                self.ld_r_r(&Register8::D, &Register8::B);
            }
            0x51 => { // LD D, C
                self.ld_r_r(&Register8::D, &Register8::C);
            }
            0x52 => { // LD D, D
                self.ld_r_r(&Register8::D, &Register8::D);
            }
            0x53 => { // LD D, E
                self.ld_r_r(&Register8::D, &Register8::E);
            }
            0x54 => { // LD D, H
                self.ld_r_r(&Register8::D, &Register8::H);
            }
            0x55 => { // LD D, L
                self.ld_r_r(&Register8::D, &Register8::L);
            }
            0x56 => { // LD D, (HL)
                self.ld_r_hl(mem, &Register8::D);
            }
            0x57 => { // LD D, A
                self.ld_r_r(&Register8::D, &Register8::A);
            }
            0x58 => { // LD E, B
                self.ld_r_r(&Register8::E, &Register8::B);
            }
            0x59 => { // LD E, C
                self.ld_r_r(&Register8::E, &Register8::C);
            }
            0x5A => { // LD E, D
                self.ld_r_r(&Register8::E, &Register8::D);
            }
            0x5B => { // LD E, E
                self.ld_r_r(&Register8::E, &Register8::E);
            }
            0x5C => { // LD E, H
                self.ld_r_r(&Register8::E, &Register8::H);
            }
            0x5D => { // LD E, L
                self.ld_r_r(&Register8::E, &Register8::L);
            }
            0x5E => { // LD E, (HL)
                self.ld_r_hl(mem, &Register8::E);
            }
            0x5F => { // LD E, A
                self.ld_r_r(&Register8::E, &Register8::A);
            }
            0x60 => { // LD H, B
                self.ld_r_r(&Register8::H, &Register8::B);
            }
            0x61 => { // LD H, C
                self.ld_r_r(&Register8::H, &Register8::C);
            }
            0x62 => { // LD H, D
                self.ld_r_r(&Register8::H, &Register8::D);
            }
            0x63 => { // LD H, E
                self.ld_r_r(&Register8::H, &Register8::E);
            }
            0x64 => { // LD H, H
                self.ld_r_r(&Register8::H, &Register8::H);
            }
            0x65 => { // LD H, L
                self.ld_r_r(&Register8::H, &Register8::L);
            }
            0x66 => { // LD H, (HL)
                self.ld_r_hl(mem, &Register8::H);
            }
            0x67 => { // LD H, A
                self.ld_r_r(&Register8::H, &Register8::A);
            }
            0x68 => { // LD L, B
                self.ld_r_r(&Register8::L, &Register8::B);
            }
            0x69 => { // LD L, C
                self.ld_r_r(&Register8::L, &Register8::C);
            }
            0x6A => { // LD L, D
                self.ld_r_r(&Register8::L, &Register8::D);
            }
            0x6B => { // LD L, E
                self.ld_r_r(&Register8::L, &Register8::E);
            }
            0x6C => { // LD L, H
                self.ld_r_r(&Register8::L, &Register8::H);
            }
            0x6D => { // LD L, L
                self.ld_r_r(&Register8::L, &Register8::L);
            }
            0x6E => { // LD L, (HL)
                self.ld_r_hl(mem, &Register8::L);
            }
            0x6F => { // LD L, A
                self.ld_r_r(&Register8::L, &Register8::A);
            }
            0x70 => { // LD (HL), B
                self.ld_hl_r(mem, &Register8::B);
            }
            0x71 => { // LD (HL), C
                self.ld_hl_r(mem, &Register8::C);
            }
            0x72 => { // LD (HL), D
                self.ld_hl_r(mem, &Register8::D);
            }
            0x73 => { // LD (HL), E
                self.ld_hl_r(mem, &Register8::E);
            }
            0x74 => { // LD (HL), H
                self.ld_hl_r(mem, &Register8::H);
            }
            0x75 => { // LD (HL), L
                self.ld_hl_r(mem, &Register8::L);
            }
            // TODO 0x76 HALT
            0x77 => { // LD (HL), A
                self.ld_hl_r(mem, &Register8::A);
            }
            0x78 => { // LD A, B
                self.ld_r_r(&Register8::A, &Register8::B);
            }
            0x79 => { // LD A, C
                self.ld_r_r(&Register8::A, &Register8::C);
            }
            0x7A => { // LD A, D
                self.ld_r_r(&Register8::A, &Register8::D);
            }
            0x7B => { // LD A, E
                self.ld_r_r(&Register8::A, &Register8::E);
            }
            0x7C => { // LD A, H
                self.ld_r_r(&Register8::A, &Register8::H);
            }
            0x7D => { // LD A, L
                self.ld_r_r(&Register8::A, &Register8::L);
            }
            0x7E => { // LD A, (HL)
                self.ld_r_hl(mem, &Register8::A);
            }
            0x7F => { // LD A, A
                self.ld_r_r(&Register8::A, &Register8::A);
            }
            0x80 => { // ADD A, B
                self.add_to_a(self.get_register_8(&Register8::B));
            }
            0x81 => { // ADD A, C
                self.add_to_a(self.get_register_8(&Register8::C));
            }
            0x82 => { // ADD A, D
                self.add_to_a(self.get_register_8(&Register8::D));
            }
            0x83 => { // ADD A, E
                self.add_to_a(self.get_register_8(&Register8::E));
            }
            0x84 => { // ADD A, H
                self.add_to_a(self.get_register_8(&Register8::H));
            }
            0x85 => { // ADD A, L
                self.add_to_a(self.get_register_8(&Register8::L));
            }
            0x86 => { // ADD A, (HL)
                let val = mem.read(self.get_register_16(&Register16::HL));
                self.add_to_a(val);
            }
            0x87 => { // ADD A, A
                self.add_to_a(self.get_register_8(&Register8::A));
            }
            0x88 => { // ADC A, B
                self.adc_to_a(self.get_register_8(&Register8::B));
            }
            0x89 => { // ADC A, C
                self.adc_to_a(self.get_register_8(&Register8::C));
            }
            0x8A => { // ADC A, D
                self.adc_to_a(self.get_register_8(&Register8::D));
            }
            0x8B => { // ADC A, E
                self.adc_to_a(self.get_register_8(&Register8::E));
            }
            0x8C => { // ADC A, H
                self.adc_to_a(self.get_register_8(&Register8::H));
            }
            0x8D => { // ADC A, L
                self.adc_to_a(self.get_register_8(&Register8::L));
            }
            0x8E => { // ADC A, (HL)
                let val = mem.read(self.get_register_16(&Register16::HL));
                self.adc_to_a(val);
            }
            0x8F => { // ADC A, A
                self.adc_to_a(self.get_register_8(&Register8::A));
            }
            0x90 => { // SUB A, B
                self.sub_from_a(self.get_register_8(&Register8::B));
            }
            0x91 => { // SUB A, C
                self.sub_from_a(self.get_register_8(&Register8::C));
            }
            0x92 => { // SUB A, D
                self.sub_from_a(self.get_register_8(&Register8::D));
            }
            0x93 => { // SUB A, E
                self.sub_from_a(self.get_register_8(&Register8::E));
            }
            0x94 => { // SUB A, H
                self.sub_from_a(self.get_register_8(&Register8::H));
            }
            0x95 => { // SUB A, L
                self.sub_from_a(self.get_register_8(&Register8::L));
            }
            0x96 => { // SUB A, (HL)
                let val = mem.read(self.get_register_16(&Register16::HL));
                self.sub_from_a(val);
            }
            0x97 => { // SUB A, A
                self.sub_from_a(self.get_register_8(&Register8::A));
            }
            0x98 => { // SBC A, B
                self.sbc_from_a(self.get_register_8(&Register8::B));
            }
            0x99 => { // SBC A, C
                self.sbc_from_a(self.get_register_8(&Register8::C));
            }
            0x9A => { // SBC A, D
                self.sbc_from_a(self.get_register_8(&Register8::D));
            }
            0x9B => { // SBC A, E
                self.sbc_from_a(self.get_register_8(&Register8::E));
            }
            0x9C => { // SBC A, H
                self.sbc_from_a(self.get_register_8(&Register8::H));
            }
            0x9D => { // SBC A, L
                self.sbc_from_a(self.get_register_8(&Register8::L));
            }
            0x9E => { // SBC A, (HL)
                let val = mem.read(self.get_register_16(&Register16::HL));
                self.sbc_from_a(val);
            }
            0x9F => { // SBC A, A
                self.sbc_from_a(self.get_register_8(&Register8::A));
            }
            0xA0 => { // AND B
                self.and_a(self.get_register_8(&Register8::B));
            }
            0xA1 => { // AND C
                self.and_a(self.get_register_8(&Register8::C));
            }
            0xA2 => { // AND D
                self.and_a(self.get_register_8(&Register8::D));
            }
            0xA3 => { // AND E
                self.and_a(self.get_register_8(&Register8::E));
            }
            0xA4 => { // AND H
                self.and_a(self.get_register_8(&Register8::H));
            }
            0xA5 => { // AND L
                self.and_a(self.get_register_8(&Register8::L));
            }
            0xA6 => { // AND (HL)
                let val = mem.read(self.get_register_16(&Register16::HL));
                self.and_a(val);
            }
            0xA7 => { // AND A
                self.and_a(self.get_register_8(&Register8::A));
            }
            0xA8 => { // XOR B
                self.xor_a(self.get_register_8(&Register8::B));
            }
            0xA9 => { // XOR C
                self.xor_a(self.get_register_8(&Register8::C));
            }
            0xAA => { // XOR D
                self.xor_a(self.get_register_8(&Register8::D));
            }
            0xAB => { // XOR E
                self.xor_a(self.get_register_8(&Register8::E));
            }
            0xAC => { // XOR H
                self.xor_a(self.get_register_8(&Register8::H));
            }
            0xAD => { // XOR L
                self.xor_a(self.get_register_8(&Register8::L));
            }
            0xAE => { // XOR (HL)
                let val = mem.read(self.get_register_16(&Register16::HL));
                self.xor_a(val);
            }
            0xAF => { // XOR A
                self.xor_a(self.get_register_8(&Register8::A));
            }
            0xB0 => { // OR B
                self.or_a(self.get_register_8(&Register8::B));
            }
            0xB1 => { // OR C
                self.or_a(self.get_register_8(&Register8::C));
            }
            0xB2 => { // OR D
                self.or_a(self.get_register_8(&Register8::D));
            }
            0xB3 => { // OR E
                self.or_a(self.get_register_8(&Register8::E));
            }
            0xB4 => { // OR H
                self.or_a(self.get_register_8(&Register8::H));
            }
            0xB5 => { // OR L
                self.or_a(self.get_register_8(&Register8::L));
            }
            0xB6 => { // OR (HL)
                let val = mem.read(self.get_register_16(&Register16::HL));
                self.or_a(val);
            }
            0xB7 => { // OR A
                self.or_a(self.get_register_8(&Register8::A));
            }
            0xB8 => { // CP B
                self.cp(self.get_register_8(&Register8::A), self.get_register_8(&Register8::B));
            }
            0xB9 => { // CP C
                self.cp(self.get_register_8(&Register8::A), self.get_register_8(&Register8::C));
            }
            0xBA => { // CP D
                self.cp(self.get_register_8(&Register8::A), self.get_register_8(&Register8::D));
            }
            0xBB => { // CP E
                self.cp(self.get_register_8(&Register8::A), self.get_register_8(&Register8::E));
            }
            0xBC => { // CP H
                self.cp(self.get_register_8(&Register8::A), self.get_register_8(&Register8::H));
            }
            0xBD => { // CP L
                self.cp(self.get_register_8(&Register8::A), self.get_register_8(&Register8::L));
            }
            0xBE => { // CP (HL)
                let val = mem.read(self.get_register_16(&Register16::HL));
                self.cp(self.get_register_8(&Register8::A), val);
            }
            0xBF => { // CP A
                self.cp(self.get_register_8(&Register8::A), self.get_register_8(&Register8::A));
            }
            0xC0 => { // RET NZ
                if self.flags.z == 0 {
                    let value = self.pop(mem);
                    self.pc = value;
                }
            }
            0xC1 => { // POP BC
                let value = self.pop(mem);
                self.set_register_16(&Register16::BC, value);
            }
            0xC2 => { // JP NZ, a16
                if self.flags.z == 0 {
                    let a16 = mem.read_16(self.pc);
                    self.pc = a16;
                } else {
                    self.pc += 2;
                }
            }
            0xC3 => { // JP a16
                let a16 = mem.read_16(self.pc);
                self.pc = a16;
            }
            0xC4 => { // CALL NZ, a16
                if self.flags.z == 0 {
                    let next = self.pc + 2;
                    self.push(mem, next);
                    self.pc = mem.read_16(self.pc);
                }
            }
            0xC5 => { // PUSH BC
                let val = self.get_register_16(&Register16::BC);
                self.push(mem, val);
            }
            0xC6 => { // ADD A, d8
                let val = mem.read(self.get_register_16(&Register16::PC));
                self.pc += 1;
                self.add_to_a(val);
            }
            0xC7 => { // RST 0
                self.push(mem, self.get_register_16(&Register16::PC));
                self.set_register_16(&Register16::PC, 0x00);
            }
            0xC8 => { // RET Z
                if self.flags.z == 1 {
                    let value = self.pop(mem);
                    self.pc = value;
                }
            }
            0xC9 => { // RET
                let value = self.pop(mem);
                self.pc = value;
            }
            0xCA => { // JP Z, a16
                if self.flags.z == 1 {
                    let a16 = mem.read_16(self.pc);
                    self.pc = a16;
                } else {
                    self.pc += 2;
                }
            }
            0xCB => { // 16-bit opcodes
                self.op_16(mem)
            }
            0xCC => { // CALL Z, a16
                if self.flags.z == 1 {
                    let next = self.pc + 2;
                    self.push(mem, next);
                    self.pc = mem.read_16(self.pc);
                }
            }
            0xCD => { // CALL a16
                let next = self.pc + 2;
                self.push(mem, next);
                self.pc = mem.read_16(self.pc);
            }
            0xCE => { // ADC A, d8
                let val = mem.read(self.pc);
                self.pc += 1;
                self.adc_to_a(val);
            }
            0xCF => { // RST 1
                self.push(mem, self.get_register_16(&Register16::PC));
                self.set_register_16(&Register16::PC, 0x08);
            }
            0xD0 => { // RET NC
                if self.flags.cy == 0 {
                    let value = self.pop(mem);
                    self.pc = value;
                }
            }
            0xD1 => { // POP DE
                let value = self.pop(mem);
                self.set_register_16(&Register16::DE, value);
            }
            0xD2 => { // JP NC, a16
                if self.flags.cy == 0 {
                    let a16 = mem.read_16(self.pc);
                    self.pc = a16;
                } else {
                    self.pc += 2;
                }
            }
            0xD4 => { // CALL NC, a16
                if self.flags.cy == 0 {
                    let next = self.pc + 2;
                    self.push(mem, next);
                    self.pc = mem.read_16(self.pc);
                }
            }
            0xD5 => { // PUSH DE
                let val = self.get_register_16(&Register16::DE);
                self.push(mem, val);
            }
            0xD6 => { // SUB d8
                let val = mem.read(self.get_register_16(&Register16::PC));
                self.pc += 1;
                self.sub_from_a(val);
            }
            0xD7 => { // RST 2
                self.push(mem, self.get_register_16(&Register16::PC));
                self.set_register_16(&Register16::PC, 0x10);
            }
            0xD8 => { // RET C
                if self.flags.cy == 1 {
                    let value = self.pop(mem);
                    self.pc = value;
                }
            }
            0xD9 => { // RETI
                self.set_interrupt(true);
                let value = self.pop(mem);
                self.pc = value;
            }
            0xDA => { // JP C, a16
                if self.flags.cy == 1 {
                    let a16 = mem.read_16(self.pc);
                    self.pc = a16;
                } else {
                    self.pc += 2;
                }
            }
            0xDC => { // CALL C, a16
                if self.flags.cy == 1 {
                    let next = self.pc + 2;
                    self.push(mem, next);
                    self.pc = mem.read_16(self.pc);
                }

            }
            0xDE => { // SBC A, d8
                let val = mem.read(self.pc);
                self.pc += 1;
                self.sbc_from_a(val);
            }
            0xDF => { // RST 3
                self.push(mem, self.get_register_16(&Register16::PC));
                self.set_register_16(&Register16::PC, 0x18);
            }
            0xE0 => { // LD (a8) A
                let val = self.get_register_8(&Register8::A);
                let addr = 0xff00 + (mem.read(self.pc) as u16);
                self.pc += 1;
                mem.write(addr, val);
            }
            0xE1 => { // POP HL
                let value = self.pop(mem);
                self.set_register_16(&Register16::HL, value);
            }
            0xE2 => { // LD (C), A
                let val = self.get_register_8(&Register8::A);
                let addr = 0xff00 + (self.get_register_8(&Register8::C) as u16);
                mem.write(addr, val);
            }
            0xE5 => { // PUSH HL
                let val = self.get_register_16(&Register16::HL);
                self.push(mem, val);
            }
            0xE6 => { // AND d8
                let val = mem.read(self.pc);
                self.pc += 1;
                self.and_a(val);
            }
            0xE7 => { // RST 4
                self.push(mem, self.get_register_16(&Register16::PC));
                self.set_register_16(&Register16::PC, 0x20);
            }
            0xE8 => { // ADD SP, s8
                let val = mem.read(self.pc);
                self.pc += 1;
                let orig = self.get_register_16(&Register16::SP);
                let sub = val & 0x80 != 0;
                let abs = (if sub {!val + 1} else {val}) as u16;
                let value = if sub {orig.wrapping_sub(abs)} else {orig.wrapping_add(abs)};
                self.set_register_16(&Register16::SP, value);
                self.flags.z = 0;
                self.flags.n = 0;
                if (orig & 0xff) + ((val as u16) & 0xff) > 0xff {
                    self.flags.h = 1;
                } else {
                    self.flags.h = 0;
                }
                if (orig as u32) + (val as u32) > 0xffff {
                    self.flags.cy = 1;
                } else {
                    self.flags.cy = 0;
                }
            }
            0xE9 => { // JP HL
                self.pc = self.get_register_16(&Register16::HL);
            }
            0xEA => { // LD (a16), A
                let val = self.get_register_8(&Register8::A);
                let addr = mem.read_16(self.pc);
                self.pc += 2;
                mem.write(addr, val);
            }
            0xEE => { // XOR d8
                let val = mem.read(self.pc);
                self.xor_a(val);
            }
            0xEF => { // RST 5
                self.push(mem, self.get_register_16(&Register16::PC));
                self.set_register_16(&Register16::PC, 0x28);
            }
            0xF0 => { // LD A, (a8)
                let a8 = 0xFF00 + (mem.read(self.pc) as u16);
                let val = mem.read(a8);
                self.set_register_8(&Register8::A, val);
            }
            0xF1 => { // POP AF
                let val = self.pop(mem);
                let upper = (val >> 8) as u8;
                self.set_register_8(&Register8::A, upper);
                let lower = (val & 0xFF) as u8;
                self.flags.z = lower>>7;
                self.flags.n = (lower & 0b01000000) >> 6;
                self.flags.h = (lower & 0b00100000) >> 5;
                self.flags.cy = (lower & 0b00010000) >> 4;
                self.flags.lower = lower & 0x0F;
            }
            0xF2 => { // LD A, (C)
                let addr = 0xFF00 + (self.get_register_8(&Register8::C) as u16);
                let val = mem.read(addr);
                self.set_register_8(&Register8::A, val);
            }
            0xF3 => { // DI
                self.set_interrupt(false);
            }
            0xF5 => { // PUSH AF
                let f = ((self.flags.z << 7) | (self.flags.n << 6) | (self.flags.h << 5) | (self.flags.cy << 4) | self.flags.lower) as u16;
                let val = ((self.get_register_8(&Register8::A) as u16) << 8) | f;
                self.push(mem, val);
            }
            0xF6 => { // OR d8
                let val = mem.read(self.pc);
                self.pc += 1;
                self.or_a(val);
            }
            0xF7 => { // RST 6
                self.push(mem, self.get_register_16(&Register16::PC));
                self.set_register_16(&Register16::PC, 0x30);
            }
            0xF8 => { // LD HL, SP+s8
                let orig = self.get_register_16(&Register16::SP);
                let s8 = mem.read(self.pc);
                self.pc += 1;
                let sub = s8 & 0x80 != 0;
                let abs = (if sub {!s8 + 1} else {s8}) as u16;
                let value = if sub {orig.wrapping_sub(abs)} else {orig.wrapping_add(abs)};
                self.set_register_16(&Register16::HL, value);
                self.flags.z = 0;
                self.flags.n = 0;
                if (orig & 0xff) + ((s8 as u16) & 0xff) > 0xff {
                    self.flags.h = 1;
                } else {
                    self.flags.h = 0;
                }
                if (orig as u32) + (s8 as u32) > 0xffff {
                    self.flags.cy = 1;
                } else {
                    self.flags.cy = 0;
                }
            }
            0xF9 => { // LD SP, HL
                let val = self.get_register_16(&Register16::HL);
                self.set_register_16(&Register16::SP, val);
            }
            0xFA => { // LD A, (a16)
                let addr = mem.read_16(self.pc);
                self.pc += 2;
                let val = mem.read(addr);
                self.set_register_8(&Register8::A, val);
            }
            0xFB => { // EI
                self.set_interrupt(true);
            }
            0xFE => { // CP d8
                let val = mem.read(self.pc);
                self.pc += 1;
                self.cp(self.get_register_8(&Register8::A), val);
            }
            0xFF => { // RST 7
                self.push(mem, self.get_register_16(&Register16::PC));
                self.set_register_16(&Register16::PC, 0x38);
            }

            _ => {
                println!("Unsupported instruction: 0x{:02x}", instruction);
                panic!("Unsupported instruction: 0x{:02x}", instruction);
            }
        }
    }

    fn op_16(&mut self, mem: &mut Memory) {
        let instruction = mem.read(self.pc);
        self.pc += 1;
        match instruction {
            0x00 => { // RLC B
                self.rlc(&Register8::B, true);
            }
            0x01 => { // RLC C
                self.rlc(&Register8::C, true);
            }
            0x02 => { // RLC D
                self.rlc(&Register8::D, true);
            }
            0x03 => { // RLC E
                self.rlc(&Register8::E, true);
            }
            0x04 => { // RLC H
                self.rlc(&Register8::H, true);
            }
            0x05 => { // RLC L
                self.rlc(&Register8::L, true);
            }
            0x06 => { // RLC (HL)
                let addr = self.get_register_16(&Register16::HL);
                let value = mem.read(addr).rotate_left(1);
                mem.write(addr, value);
                self.flags.cy = value & 0x01;
                self.flags.h = 0;
                self.flags.z = if value == 0 {1} else {0};
                self.flags.n = 0;
            }
            0x07 => { // RLC A
                self.rlc(&Register8::A, true);
            }
            0x08 => { // RRC B
                self.rrc(&Register8::B, true);
            }
            0x09 => { // RRC C
                self.rrc(&Register8::C, true);
            }
            0x0A => { // RRC D
                self.rrc(&Register8::D, true);
            }
            0x0B => { // RRC E
                self.rrc(&Register8::E, true);
            }
            0x0C => { // RRC H
                self.rrc(&Register8::H, true);
            }
            0x0D => { // RRC L
                self.rrc(&Register8::L, true);
            }
            0x0E => { // RRC (HL)
                let addr = self.get_register_16(&Register16::HL);
                let value = mem.read(addr).rotate_right(1);
                mem.write(addr, value);
                self.flags.cy = value>>7;
                self.flags.h = 0;
                self.flags.z = if value == 0 {1} else {0};
                self.flags.n = 0;
            }
            0x0F => { // RRC A
                self.rrc(&Register8::A, true);
            }
            0x10 => { // RL B
                self.rl(&Register8::B, true);
            }
            0x11 => { // RL C
                self.rl(&Register8::C, true);
            }
            0x12 => { // RL D
                self.rl(&Register8::D, true);
            }
            0x13 => { // RL E
                self.rl(&Register8::E, true);
            }
            0x14 => { // RL H
                self.rl(&Register8::H, true);
            }
            0x15 => { // RL L
                self.rl(&Register8::L, true);
            }
            0x16 => { // RL (HL)
                let addr = self.get_register_16(&Register16::HL);
                let orig = mem.read(addr);
                let mut value = orig << 1;
                value = value | self.flags.cy;
                self.flags.cy = orig >> 7;
                mem.write(addr, value);
                self.flags.h = 0;
                self.flags.z = if value == 0 {1} else {0};
                self.flags.n = 0;
            }
            0x17 => { // RL A
                self.rl(&Register8::A, true);
            }
            0x18 => { // RR B
                self.rr(&Register8::B, true);
            }
            0x19 => { // RR C
                self.rr(&Register8::C, true);
            }
            0x1A => { // RR D
                self.rr(&Register8::D, true);
            }
            0x1B => { // RR E
                self.rr(&Register8::E, true);
            }
            0x1C => { // RR H
                self.rr(&Register8::H, true);
            }
            0x1D => { // RR L
                self.rr(&Register8::L, true);
            }
            0x1E => { // RR (HL)
                let addr = self.get_register_16(&Register16::HL);
                let orig = mem.read(addr);
                let mut value = orig >> 1;
                value = (value & 0b01111111) | (self.flags.cy << 7);
                self.flags.cy = orig & 0x01;
                mem.write(addr, value);
                self.flags.h = 0;
                self.flags.z = if value == 0 {1} else {0};
                self.flags.n = 0;
            }
            0x1F => { // RR A
                self.rr(&Register8::A, true);
            }
            0x20 => { // SLA B
                self.sl(&Register8::B);
            }
            0x21 => { // SLA C
                self.sl(&Register8::C);
            }
            0x22 => { // SLA D
                self.sl(&Register8::D);
            }
            0x23 => { // SLA E
                self.sl(&Register8::E);
            }
            0x24 => { // SLA H
                self.sl(&Register8::H);
            }
            0x25 => { // SLA L
                self.sl(&Register8::L);
            }
            0x26 => { // SLA (HL)
                let addr = self.get_register_16(&Register16::HL);
                let orig = mem.read(addr);
                let val = orig << 1;
                mem.write(addr,val);
                self.flags.cy = val >> 7;
                self.flags.z = if val == 0 {1} else {0};
                self.flags.n = 0;
                self.flags.h = 0;
            }
            0x27 => { // SLA A
                self.sl(&Register8::A);
            }
            0x28 => { // SRA B
                self.sra(&Register8::B);
            }
            0x29 => { // SRA C
                self.sra(&Register8::C);
            }
            0x2A => { // SRA D
                self.sra(&Register8::D);
            }
            0x2B => { // SRA E
                self.sra(&Register8::E);
            }
            0x2C => { // SRA H
                self.sra(&Register8::H);
            }
            0x2D => { // SRA L
                self.sra(&Register8::L);
            }
            0x2E => { // SRA (HL)
                let addr = self.get_register_16(&Register16::HL);
                let orig = mem.read(addr);
                let val = (orig >> 1) | (orig & 0x80);
                mem.write(addr,val);
                self.flags.cy = val & 0x01;
                self.flags.z = if val == 0 {1} else {0};
                self.flags.n = 0;
                self.flags.h = 0;
            }
            0x2F => { // SRA A
                self.sra(&Register8::A);
            }
            0x30 => { // SWAP B
                self.swap(&Register8::B);
            }
            0x31 => { // SWAP C
                self.swap(&Register8::C);
            }
            0x32 => { // SWAP D
                self.swap(&Register8::D);
            }
            0x33 => { // SWAP E
                self.swap(&Register8::E);
            }
            0x34 => { // SWAP H
                self.swap(&Register8::H);
            }
            0x35 => { // SWAP L
                self.swap(&Register8::L);
            }
            0x36 => { // SWAP (HL)
                let addr = self.get_register_16(&Register16::HL);
                let orig = mem.read(addr);
                let value = (orig << 4) | (orig >> 4);
                mem.write(addr,value);
                self.flags.z = if value == 0 {1} else {0};
                self.flags.cy = 0;
                self.flags.n = 0;
                self.flags.h = 0;
            }
            0x37 => { // SWAP A
                self.swap(&Register8::A);
            }
            0x38 => { // SRL B
                self.srl(&Register8::B);
            }
            0x39 => { // SRL C
                self.srl(&Register8::C);
            }
            0x3A => { // SRL D
                self.srl(&Register8::D);
            }
            0x3B => { // SRL E
                self.srl(&Register8::E);
            }
            0x3C => { // SRL H
                self.srl(&Register8::H);
            }
            0x3D => { // SRL L
                self.srl(&Register8::L);
            }
            0x3E => { // SRL (HL)
                let addr = self.get_register_16(&Register16::HL);
                let orig = mem.read(addr);
                let val = orig >> 1;
                mem.write(addr,val);
                self.flags.cy = val & 0x01;
                self.flags.z = if val == 0 {1} else {0};
                self.flags.n = 0;
                self.flags.h = 0;
            }
            0x3F => { // SRL A
                self.srl(&Register8::A);
            }
            0x40 | 0x48 | 0x50 | 0x58 | 0x60 | 0x68 | 0x70 | 0x78 => { // BIT B
                self.bit(&Register8::B, (instruction - 0x40) / 0x08);
            }
            0x41 | 0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x71 | 0x79 => { // BIT C
                self.bit(&Register8::C, (instruction - 0x41) / 0x08);
            }
            0x42 | 0x4A | 0x52 | 0x5A | 0x62 | 0x6A | 0x72 | 0x7A => { // BIT D
                self.bit(&Register8::D, (instruction - 0x42) / 0x08);
            }
            0x43 | 0x4B | 0x53 | 0x5B | 0x63 | 0x6B | 0x73 | 0x7B => { // BIT E
                self.bit(&Register8::E, (instruction - 0x43) / 0x08);
            }
            0x44 | 0x4C | 0x54 | 0x5C | 0x64 | 0x6C | 0x74 | 0x7C => { // BIT H
                self.bit(&Register8::H, (instruction - 0x44) / 0x08);
            }
            0x45 | 0x4D | 0x55 | 0x5D | 0x65 | 0x6D | 0x75 | 0x7D => { // BIT L
                self.bit(&Register8::L, (instruction - 0x45) / 0x08);
            }
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x76 | 0x7E => { // BIT (HL)
                let bit = (instruction - 0x46) / 0x08;
                let addr = self.get_register_16(&Register16::HL);
                let val = mem.read(addr);
                self.flags.z = if (val & (1 << bit)) != 0 {1} else {0};
                self.flags.n = 0;
                self.flags.h = 1;
            }
            0x47 | 0x4F | 0x57 | 0x5F | 0x67 | 0x6F | 0x77 | 0x7F => { // BIT A
                self.bit(&Register8::A, (instruction - 0x47) / 0x08);
            }
            0x80 | 0x88 | 0x90 | 0x98 | 0xA0 | 0xA8 | 0xB0 | 0xB8 => { // RES B
                self.res(&Register8::B, (instruction - 0x80) / 0x08);
            }
            0x81 | 0x89 | 0x91 | 0x99 | 0xA1 | 0xA9 | 0xB1 | 0xB9 => { // RES C
                self.res(&Register8::C, (instruction - 0x81) / 0x08);
            }
            0x82 | 0x8A | 0x92 | 0x9A | 0xA2 | 0xAA | 0xB2 | 0xBA => { // RES D
                self.res(&Register8::D, (instruction - 0x82) / 0x08);
            }
            0x83 | 0x8B | 0x93 | 0x9B | 0xA3 | 0xAB | 0xB3 | 0xBB => { // RES E
                self.res(&Register8::E, (instruction - 0x83) / 0x08);
            }
            0x84 | 0x8C | 0x94 | 0x9C | 0xA4 | 0xAC | 0xB4 | 0xBC => { // RES H
                self.res(&Register8::H, (instruction - 0x84) / 0x08);
            }
            0x85 | 0x8D | 0x95 | 0x9D | 0xA5 | 0xAD | 0xB5 | 0xBD => { // RES L
                self.res(&Register8::L, (instruction - 0x85) / 0x08);
            }
            0x86 | 0x8E | 0x96 | 0x9E | 0xA6 | 0xAE | 0xB6 | 0xBE => { // RES (HL)
                let bit = (instruction - 0x86) / 0x08;
                let addr = self.get_register_16(&Register16::HL);
                let orig = mem.read(addr);
                let val = orig & !(1 << bit);
                mem.write(addr,val);
            }
            0x87 | 0x8F | 0x97 | 0x9F | 0xA7 | 0xAF | 0xB7 | 0xBF => { // RES A
                self.res(&Register8::A, (instruction - 0x87) / 0x08);
            }
            0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xE0 | 0xE8 | 0xF0 | 0xF8 => { // SET B
                self.set(&Register8::B, (instruction - 0xC0) / 0x08);
            }
            0xC1 | 0xC9 | 0xD1 | 0xD9 | 0xE1 | 0xE9 | 0xF1 | 0xF9 => { // SET C
                self.set(&Register8::C, (instruction - 0xC1) / 0x08);
            }
            0xC2 | 0xCA | 0xD2 | 0xDA | 0xE2 | 0xEA | 0xF2 | 0xFA => { // SET D
                self.set(&Register8::D, (instruction - 0xC2) / 0x08);
            }
            0xC3 | 0xCB | 0xD3 | 0xDB | 0xE3 | 0xEB | 0xF3 | 0xFB => { // SET E
                self.set(&Register8::E, (instruction - 0xC3) / 0x08);
            }
            0xC4 | 0xCC | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => { // SET H
                self.set(&Register8::H, (instruction - 0xC4) / 0x08);
            }
            0xC5 | 0xCD | 0xD5 | 0xDD | 0xE5 | 0xED | 0xF5 | 0xFD => { // SET L
                self.set(&Register8::L, (instruction - 0xC5) / 0x08);
            }
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => { // SET (HL)
                let bit = (instruction - 0xC6) / 0x08;
                let addr = self.get_register_16(&Register16::HL);
                let orig = mem.read(addr);
                let val = orig | (1 << bit);
                mem.write(addr,val);
            }
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => { // SET A
                self.set(&Register8::A, (instruction - 0xC7) / 0x08);
            }

            _ => {
                println!("Unsupported instruction: 0xCB{:02x}", instruction);
                panic!("Unsupported instruction: 0xCB{:02x}", instruction);
            }
        }

    }
}

struct Flags {
    z: u8,
    n: u8,
    h: u8,
    cy: u8,
    lower: u8,
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
    use crate::cpu::{CPU, Register, Register16, Register8};
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
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.a = 0x01 << 7;
            mem.write(0, 0x07);
            cpu.run(&mut mem);

            assert_eq!(cpu.flags.cy, 1);
            assert_eq!(cpu.a, 0x01);
    }

    #[test]
    fn cpu_rrca() {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.a = 0x01;
            mem.write(0, 0x0F);
            cpu.run(&mut mem);

            assert_eq!(cpu.flags.cy, 1);
            assert_eq!(cpu.a, 0x01 << 7);
    }

    #[test]
    fn cpu_rla() {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.a = 0x01 << 7;
            mem.write(0, 0x17);
            cpu.run(&mut mem);

            assert_eq!(cpu.flags.cy, 1);
            assert_eq!(cpu.a, 0x00);
    }

    #[test]
    fn cpu_rra() {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.a = 0x01;
            mem.write(0, 0x1F);
            cpu.run(&mut mem);

            assert_eq!(cpu.flags.cy, 1);
            assert_eq!(cpu.a, 0x00);
    }

    #[test]
    fn cpu_jr() {
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

    #[test]
    fn cpu_push_pop() {
            let mut mem = Memory::new();
            let mut cpu = CPU::new();
            cpu.simulate_bootloader();
            mem.simulate_bootloader();
            cpu.set_register_16(&Register16::BC, 99);
            mem.write(cpu.get_register_16(&Register16::PC), 0xC5);
            mem.write(cpu.get_register_16(&Register16::PC) + 1, 0xC1);
            cpu.run(&mut mem);
            cpu.set_register_16(&Register16::BC, 5);
            assert_eq!(cpu.get_register_16(&Register16::BC), 5);
            cpu.run(&mut mem);
            assert_eq!(cpu.get_register_16(&Register16::BC), 99);
    }

    #[test]
    fn cpu_res() {
        let mut mem = Memory::new();
        let mut cpu = CPU::new();
        cpu.set_register_8(&Register8::B, 0b00100000);
        cpu.set_register_8(&Register8::C, 0b00100000);
        cpu.set_register_8(&Register8::D, 0b00100000);
        cpu.set_register_8(&Register8::E, 0b00100000);
        cpu.set_register_8(&Register8::H, 0b00100000);
        cpu.set_register_8(&Register8::L, 0b00100000);
        cpu.set_register_8(&Register8::A, 0b00100000);
        mem.write(0, 0xCB);
        mem.write(1, 0xA8);
        mem.write(2, 0xCB);
        mem.write(3, 0xA9);
        mem.write(4, 0xCB);
        mem.write(5, 0xAA);
        mem.write(6, 0xCB);
        mem.write(7, 0xAB);
        mem.write(8, 0xCB);
        mem.write(9, 0xAC);
        mem.write(10, 0xCB);
        mem.write(11, 0xAD);
        mem.write(12, 0xCB);
        mem.write(13, 0xAF);
        for _ in 0..=6 {
            cpu.run(&mut mem);
        }

        assert_eq!(cpu.get_register_8(&Register8::B), 0);
        assert_eq!(cpu.get_register_8(&Register8::C), 0);
        assert_eq!(cpu.get_register_8(&Register8::D), 0);
        assert_eq!(cpu.get_register_8(&Register8::E), 0);
        assert_eq!(cpu.get_register_8(&Register8::H), 0);
        assert_eq!(cpu.get_register_8(&Register8::L), 0);
        assert_eq!(cpu.get_register_8(&Register8::A), 0);
    }

    #[test]
    fn cpu_set() {
        let mut mem = Memory::new();
        let mut cpu = CPU::new();
        mem.write(0, 0xCB);
        mem.write(1, 0xF8);
        mem.write(2, 0xCB);
        mem.write(3, 0xF9);
        mem.write(4, 0xCB);
        mem.write(5, 0xFA);
        mem.write(6, 0xCB);
        mem.write(7, 0xFB);
        mem.write(8, 0xCB);
        mem.write(9, 0xFC);
        mem.write(10, 0xCB);
        mem.write(11, 0xFD);
        mem.write(12, 0xCB);
        mem.write(13, 0xFF);
        for _ in 0..=6 {
            cpu.run(&mut mem);
        }

        assert_eq!(cpu.get_register_8(&Register8::B), 0x80);
        assert_eq!(cpu.get_register_8(&Register8::C), 0x80);
        assert_eq!(cpu.get_register_8(&Register8::D), 0x80);
        assert_eq!(cpu.get_register_8(&Register8::E), 0x80);
        assert_eq!(cpu.get_register_8(&Register8::H), 0x80);
        assert_eq!(cpu.get_register_8(&Register8::L), 0x80);
        assert_eq!(cpu.get_register_8(&Register8::A), 0x80);
    }
}
