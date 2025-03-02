use marble::evaluate_string;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(s: &str) -> String {
    match evaluate_string(s) {
        Ok(value) => value.to_string(),
        Err(e) => format!("Error -> {e}"),
    }
}