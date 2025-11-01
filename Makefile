build:
	@ wasm-pack build --target web --no-typescript --release --out-dir extension/wasm

build-dev:
	@ wasm-pack build --target web --no-typescript --out-dir extension/wasm

test:
	@ cargo test