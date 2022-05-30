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

// TODO: We should have a proper Result type here so we can pass error messages up to javascript to
// display it to the user. Until then, the user has to look into the console for the actual error.
#[wasm_bindgen]
pub fn convert_preset(s : &str) -> Option<String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    preset_converter::convert_preset(s)
}
