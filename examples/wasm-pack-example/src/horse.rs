use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&format!("Hello from {}", file!()).into());
}
