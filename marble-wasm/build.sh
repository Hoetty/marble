rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir ../marble-editor/wasm --target web ./target/wasm32-unknown-unknown/release/marble_wasm.wasm