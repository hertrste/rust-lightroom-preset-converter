mod preset_converter;
mod utils;

use wasm_bindgen::prelude::*;
use std::panic;

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
    alert("Hello, preset-converter!");
}


#[wasm_bindgen]
pub fn convert_preset(s : &str) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    preset_converter::convert_preset(s)
}
