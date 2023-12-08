@ECHO OFF
cargo wasm
wasm-bindgen --target web --no-typescript --out-name dat --out-dir wasm target/wasm32-unknown-unknown/wasm-release/destruction_and_tranquility.wasm
@REM --enable-bulk-memory --enable-threads
wasm-opt -O -ol 100 -s 100 -o wasm/dat_bg.wasm wasm/dat_bg.wasm
@REM cargo install http-server
http-server --cors --gzip --port 8080 wasm
