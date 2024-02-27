pub mod parser;
pub mod formatter;

use wasm_bindgen::prelude::*;

// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {}

#[wasm_bindgen]
#[cfg(feature = "benchmark")]
pub fn parse(source: String) -> String {
    web_sys::console::time_with_label("parse_time");

    let obj = parser::parse(source);
    let f = formatter::format(obj);

    web_sys::console::time_end_with_label("parse_time");

    f
}

#[wasm_bindgen]
#[cfg(not(feature = "benchmark"))]
pub fn parse(source: String) -> String {
    let obj = parser::parse(source);
    formatter::format(obj)
}