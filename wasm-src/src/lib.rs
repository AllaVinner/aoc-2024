mod aoc_solver;
mod days;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-src!");
}

#[wasm_bindgen(js_name=test_fn)]
pub fn test_ok() -> Result<String, String> {
    return Ok("Was successfull".to_string());
}

#[wasm_bindgen(js_name=test_err)]
pub fn test_err() -> Result<String, String> {
    return Err("Was error".to_string());
}

#[wasm_bindgen(js_name=solve)]
pub fn solve(input: String, day: i32, part: i32) -> Result<String, String> {
    return aoc_solver::solve(&input, day, part);
}
