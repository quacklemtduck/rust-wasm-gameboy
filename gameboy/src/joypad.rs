use web_sys::console;

use crate::memory::Memory;

// Implementation inspired and based on https://github.com/torch2424/wasmboy

pub struct Joypad {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
    a: bool,
    b: bool,
    select: bool,
    start: bool,

    is_dpad_type: bool,
    is_button_type: bool,
    joypad_register_flipped: u8
}

enum Button {
    Up,
    Right,
    Down,
    Left,
    A,
    B,
    Select,
    Start
}

impl Joypad {

    pub fn new() -> Joypad {
        Joypad { up: false, right: false, down: false, left: false, a: false, b: false, select: false, start: false, is_dpad_type: false, is_button_type: false, joypad_register_flipped: 0 }
    }

    pub fn update_joypad(&mut self, value: u8) {
        //console::log_1(&format!("Keyboard value {:#b} A: {} B: {}", value, self.a, self.b).into());
        self.joypad_register_flipped = value ^ 0xFF;
        self.is_dpad_type = self.joypad_register_flipped & 0b10000 > 0;
        self.is_button_type = self.joypad_register_flipped & 0b100000 > 0;
    }

    pub fn set_joypad_state(&mut self, up: i32, right: i32, down: i32, left: i32, a: i32, b: i32, select: i32, start: i32) -> bool {
        let mut request_interrupt = false;
        if up > 0 {
            request_interrupt = self.press_button(&Button::Up);
        } else {
            self.release_button(&Button::Up);
        }

        if right > 0 {
            request_interrupt = self.press_button(&Button::Right);
        } else {
            self.release_button(&Button::Right);
        }

        if down > 0 {
            request_interrupt = self.press_button(&Button::Down);
        } else {
            self.release_button(&Button::Down);
        }

        if left > 0 {
            request_interrupt = self.press_button(&Button::Left);
        } else {
            self.release_button(&Button::Left);
        }

        if a > 0 {
            request_interrupt = self.press_button(&Button::A);
        } else {
            self.release_button(&Button::A);
        }

        if b > 0 {
            request_interrupt = self.press_button(&Button::B);
        } else {
            self.release_button(&Button::B);
        }

        if select > 0 {
            request_interrupt = self.press_button(&Button::Select);
        } else {
            self.release_button(&Button::Select);
        }

        if start > 0 {
            request_interrupt = self.press_button(&Button::Start);
        } else {
            self.release_button(&Button::Start);
        }

        return request_interrupt
    }

    pub fn get_joypad_state(&self) -> u8 {

        let mut register = self.joypad_register_flipped;

        if self.is_dpad_type {
            if self.up {
                register = register & 0b11111011
            } else {
                register = register | 0b100
            }

            if self.right {
                register = register & 0b11111110
            } else {
                register = register | 0b1
            }

            if self.down {
                register = register & 0b11110111
            } else {
                register = register | 0b1000
            }

            if self.left {
                register = register & 0b11111101
            } else {
                register = register | 0b10
            }
        } else if self.is_button_type {
            
            if self.a {
                register = register & 0b11111110
            } else {
                register = register | 0b1
            }

            if self.b {
                register = register & 0b11111101
            } else {
                register = register | 0b10
            }

            if self.select {
                register = register & 0b11111011
            } else {
                register = register | 0b100
            }

            if self.start {
                register = register & 0b11110111
            } else {
                register = register | 0b1000
            }
        }

        // Setting the top 4 bits
        register = register | 0xf0;

        return register;
    }

    fn press_button(&mut self, button: &Button) -> bool {
        let mut is_button_state_changing = false;
        if !self.get_button_state(button) {
            is_button_state_changing = true;
        }

        self.set_button_state(button, true);

        if is_button_state_changing {
            let is_dpad_button = match button {
                Button::Up | Button::Right | Button::Down | Button::Left => true,
                _ => false
            };

            let mut  should_request_interrupt = false;
            
            if is_dpad_button && self.is_dpad_type {
                should_request_interrupt = true;
            }

            if !is_dpad_button && self.is_button_type {
                should_request_interrupt = true;
            }

            return should_request_interrupt;
        }

        return false;
    }

    fn release_button(&mut self, button: &Button) {
        self.set_button_state(button, false)
    }

    fn get_button_state(&self, button: &Button) -> bool{
        match button {
            Button::Up => self.up,
            Button::Right => self.right,
            Button::Down => self.down,
            Button::Left => self.left,
            Button::A => self.a,
            Button::B => self.b,
            Button::Select => self.select,
            Button::Start => self.start,
        }
    }

    fn set_button_state(&mut self, button: &Button, is_pressed: bool) {
        match button {
            Button::Up => self.up = is_pressed,
            Button::Right => self.right = is_pressed,
            Button::Down => self.down = is_pressed,
            Button::Left => self.left = is_pressed,
            Button::A => self.a = is_pressed,
            Button::B => self.b = is_pressed,
            Button::Select => self.select = is_pressed,
            Button::Start => self.start = is_pressed,
        }
    }
}