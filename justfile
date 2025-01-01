set windows-shell := ["pwsh.exe", "-c"]
# renovate: datasource=crate depName=cargo-nextest packageName=cargo-nextest
NEXTEST_VERSION := "0.9.72"
# renovate: datasource=crate depName=cargo-component packageName=cargo-component
CARGO_COMPONENT_VERSION := "0.19.0"
# renovate: datasource=crate depName=sd packageName=sd
SD_VERSION := "1.0.0"
# renovate: datasource=crate depName=cargo-llvm-cov packageName=cargo-llvm-cov
CARGO_LLVM_COV_VERSION := "0.6.13"
# renovate: datasource=crate depName=cargo-hack packageName=cargo-hack
CARGO_HACK_VERSION := "0.6.33"

@default: build-language-plugin regenerator install-requirements build-wasm-components build-wasm-components-release test-all rust-docs fmt

# Checks formatting and regenerator
househeeping-ci-flow: regenerator fmt

# Runs all amd64 unit and doc tests tests
base-ci-flow: test

# Runs all examples/*
examples-ci-flow: build-language-plugin build-wasm-components build-wasm-components-release test-examples

# Regenerates provider from generator's integration test
generator-ci-flow COMPILATION_NAME:
    just test-provider-compilation {{COMPILATION_NAME}}

# Test docs examples and creates docs
test-docs-ci-flow: test-docs

# https://stackoverflow.com/questions/74524817/why-is-anyhow-not-working-in-the-stable-version
fix-issues:
    cargo component check
    cargo check

build-language-plugin:
    cd pulumi-language-wasm && just

package-language-plugin VERSION:
    cd pulumi-language-wasm && just package-language-plugin-all {{VERSION}}

install-requirements:
    rustup component add rustfmt
    rustup component add llvm-tools-preview
    cargo binstall --no-confirm cargo-nextest@{{NEXTEST_VERSION}}
    cargo binstall --no-confirm cargo-component@{{CARGO_COMPONENT_VERSION}}
    cargo binstall --no-confirm sd@{{SD_VERSION}}
    cargo binstall --no-confirm cargo-llvm-cov@{{CARGO_LLVM_COV_VERSION}}
    cargo binstall --no-confirm cargo-hack@{{CARGO_HACK_VERSION}}

# Compiling everything together causes linking issues
build-wasm-components:
    cargo build -p pulumi_wasm_runner
    cargo component build -p pulumi_wasm
    cargo component build -p pulumi_wasm_example_simple
    cargo component build -p pulumi_wasm_example_docker
    cargo component build -p pulumi_wasm_example_dependencies
    cargo component build -p pulumi_wasm_example_multiple_providers

build-wasm-components-release:
    cargo build -p pulumi_wasm_runner --release
    cargo component build -p pulumi_wasm --release
    cargo component build -p pulumi_wasm_example_simple --release
    cargo component build -p pulumi_wasm_example_docker --release
    cargo component build -p pulumi_wasm_example_dependencies --release
    cargo component build -p pulumi_wasm_example_multiple_providers --release

check:
    cargo fmt -- --check

fmt:
    cd pulumi-language-wasm && just fmt
    cargo fmt

fmt-clippy:
    cargo clippy --all-features --fix --allow-dirty --allow-staged
    just fmt

clippy-to-file:
    cargo clippy --all-features --message-format=json | clippy-sarif | tee rust-clippy-results.sarif | sarif-fmt

regenerator:
    cargo run -p regenerator

publish:
    cargo hack publish -p pulumi_wasm_wit --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_wasm_proto --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_wasm_common --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_wasm_rust_macro --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_wasm_rust --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_wasm_generator --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_wasm_build --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_wasm_core --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_wasm_runner_component_creator --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_wasm_runner --all-features --no-dev-deps --allow-dirty

test-provider-compilation COMPILATION_NAME:
    cargo llvm-cov nextest -p pulumi_wasm_generator --cobertura --output-path covertura.xml --features generator_{{COMPILATION_NAME}} --test '*'

test-examples:
    cargo llvm-cov nextest \
        -p pulumi_wasm_example_simple \
        -p pulumi_wasm_example_docker \
        -p pulumi_wasm_example_dependencies \
        -p pulumi_wasm_example_multiple_providers \
        -p pulumi_wasm_example_typesystem \
        --cobertura --output-path covertura.xml --features example_test

test-all:
    cargo llvm-cov nextest --workspace --cobertura --output-path covertura.xml --all-features

test:
    cargo llvm-cov nextest --cobertura --output-path covertura.xml

docs:
    docker run --rm -it -p 8000:8000 -v ${PWD}:/docs squidfunk/mkdocs-material

test-docs:
    cargo test --doc --workspace
    just rust-docs

# DO NOT EDIT - GENERATE-RUST-DOCS - START
rust-docs:
    cargo doc --no-deps -p pulumi_wasm_rust -p pulumi_wasm_build -p pulumi_wasm_providers_cloudflare -p pulumi_wasm_providers_docker -p pulumi_wasm_providers_random
# DO NOT EDIT - GENERATE-RUST-DOCS - END

update-version NEW_VERSION:
    sd "0.0.0-DEV" "{{NEW_VERSION}}" "pulumi_wasm_wit/wit/world.wit" "pulumi_wasm_rust_macro/src/lib.rs" \
    "Cargo.toml"
