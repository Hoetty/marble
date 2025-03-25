use marble::{execute_string, source::Source};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(s: &str) -> String {
    match execute_string(s) {
        Ok((value, output)) => format!("{output}{value}"),
        Err(e) => e.of_source(&Source::new(s)),
    }
}