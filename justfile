set windows-shell := ["pwsh.exe", "-c"]

@default: build test

build: build-language-plugin regenerate-providers install-requirements build-wasm-components fmt

build-language-plugin:
    cd pulumi-language-wasm && just

install-requirements:
    rustup component add rustfmt
    cargo install cargo-nextest@0.9.68 --locked || cargo-nextest --version
    cargo install cargo-component@0.10.1 --locked || cargo-component --version
    cargo install wasm-tools@1.202.0 --locked || wasm-tools --version

build-wasm-components:
    cargo component build -p pulumi_wasm -p pulumi_wasm_random_provider -p pulumi_wasm_example_simple
    cargo run -p cargo-pulumi -- -p pulumi_wasm_example_simple

check:
    cargo fmt --all -- --check

fmt:
    cd pulumi-language-wasm && just fmt
    cargo fmt --all

fmt-clippy:
    cd pulumi-language-wasm && just fmt
    cargo clippy --all --all-features --fix --allow-dirty --allow-staged
    cargo fmt --all

regenerate-providers:
    cargo run -p cargo-pulumi-gen -- gen-provider --remove true --schema providers/random.json --output providers/pulumi_wasm_provider_random
    cargo run -p cargo-pulumi-gen -- gen-rust     --remove true --schema providers/random.json --output providers/pulumi_wasm_provider_random_rust

test:
    cargo nextest run --all

docs:
    docker run --rm -it -p 8000:8000 -v ${PWD}:/docs squidfunk/mkdocs-material