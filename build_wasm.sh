rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --package marble-wasm --target wasm32-unknown-unknown 
wasm-bindgen --out-dir marble-editor/wasm --target web target/wasm32-unknown-unknown/release/marble_wasm.wasm