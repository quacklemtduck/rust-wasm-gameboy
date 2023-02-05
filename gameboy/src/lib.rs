extern crate core;

mod cpu;
mod memory;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, gameboy!");
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a * 2 + b * 2
}

#[wasm_bindgen]
pub fn dec(a: i32, b: i32) -> i32 {
    a - b - b
}