#!/usr/bin/env bash

set -e

./run.sh

cargo run -p pulumi_wasm_runner -- run --wasm target/wasm32-wasi/debug/composed2.wasm
