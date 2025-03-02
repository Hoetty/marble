SCRIPT_DIR=$(dirname "$(realpath "$0")")
cd "$SCRIPT_DIR" || exit

rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir ../marble-editor/wasm --target web ./target/wasm32-unknown-unknown/release/marble_wasm.wasm