@REM .\run.bat && cargo run -- --cwasm target/wasm32-wasi/debug/composed2.cwasm run

@REM cargo run -- run --cwasm target/wasm32-wasi/debug/composed2.cwasm

@REM cargo run -- run --wasm target/wasm32-wasi/debug/composed2.wasm

.\build.bat && cargo run -p pulumi_wasm_runner -- run --wasm target/wasm32-wasi/debug/composed2.wasm