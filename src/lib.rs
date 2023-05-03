mod models;
pub mod cui;
pub mod core;
pub mod wasm;

pub use models::*;

use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    core::start();
}
