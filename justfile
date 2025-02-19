
build:
    wasm-pack build --target web --out-dir web

watch:
    cargo watch -w src -- just build