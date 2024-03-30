set windows-shell := ["pwsh.exe", "-c"]

@default: build test

build: build-language-plugin regenerate-providers install-requirements build-wasm-components build-libraries fmt

build-language-plugin:
    cd pulumi-language-wasm && just

install-requirements:
    rustup component add rustfmt
    cargo install cargo-component@0.10.1 --locked || cargo-component --version
    cargo install wasm-tools@1.201.0 --locked || wasm-tools --version

build-wasm-components:
    cargo component build -p pulumi_wasm -p pulumi_wasm_random_provider -p pulumi_wasm_example_simple

build-libraries:
    cargo build -p pulumi_wasm_runner \
                -p pulumi_wasm_rust \
                -p pulumi_wasm_rust_macro \
                -p pulumi_wasm_random

check:
    cargo fmt --all -- --check

fmt:
    cargo fmt --all

format:
    cargo fmt --all

format-clippy:
    cargo clippy --all --all-features --fix --allow-dirty --allow-staged
    cargo fmt --all

regenerate-providers:
    cargo run -p cargo-pulumi-gen -- gen-provider --remove true --schema providers/random.json --output providers/pulumi_wasm_provider_random
    cargo run -p cargo-pulumi-gen -- gen-rust     --remove true --schema providers/random.json --output providers/pulumi_wasm_provider_random_rust

test:
    cargo test --all
