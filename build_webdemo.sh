cargo build --release -p boseiju_wasm --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/boseiju_wasm.wasm ./boseiju_webdemo/boseiju.wasm
cargo run --release --bin export_for_webdemo > boseiju_webdemo/scripts/example_cards.js

# to run a quick server:
# python3 -m http.server --directory boseiju_webdemo
