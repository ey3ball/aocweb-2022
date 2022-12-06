mod utils;
use wasm_bindgen::prelude::*;
use aoclib::hello;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let message = hello();
    alert(&format!("Hello ?? {}, aocweb!", message));
}

pub use aoclib::day01;
pub use utils::set_panic_hook;
