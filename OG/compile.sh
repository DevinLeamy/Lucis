trunk build

# build an unoptimized wasm module 
RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
    cargo build --release --target wasm32-unknown-unknown \
    -Z build-std=std,panic_abort
# optimize the constructed wasm module
# TODO: might be able to use the newer "--target web"
wasm-bindgen target/wasm32-unknown-unknown/release/Lucis.wasm \
    --out-dir dist \
    --target no-modules

# cp index.html dist
cp index.js dist
cp worker.js dist

python3 server.py
