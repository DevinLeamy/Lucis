rm -rf dist
rm -rf pkg

cd ../glue

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
    cargo build --release --target wasm32-unknown-unknown \
    -Z build-std=std,panic_abort

wasm-bindgen target/wasm32-unknown-unknown/release/glue.wasm \
    --out-dir ../client/dist \
    --target no-modules

cd ../client

cp ../glue/worker.js ./dist

npx webpack
