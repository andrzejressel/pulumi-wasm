cargo run -p cargo-pulumi
if(!$?) { Exit $LASTEXITCODE }

cargo run -p pulumi_wasm_runner -- run --wasm ../../target/wasm32-wasi/debug/composed.wasm
if(!$?) { Exit $LASTEXITCODE }
