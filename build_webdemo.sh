cargo build --release -p boseiju_wasm --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/boseiju_wasm.wasm ./boseiju_webdemo/boseiju.wasm

# to run a quick server:
# python3 -m http.server --directory boseiju_webdemo
