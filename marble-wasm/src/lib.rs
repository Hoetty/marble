use std::path::PathBuf;

use marble::{execute_string, source::Source};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(s: &str) -> String {
    match execute_string(s, PathBuf::default()) {
        Ok((value, output)) => format!("{output}{value}"),
        Err(e) => e.of_source(&Source::new(s)),
    }
}
