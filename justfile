set windows-shell := ["pwsh.exe", "-c"]
# renovate: datasource=crate depName=cargo-nextest packageName=cargo-nextest
NEXTEST_VERSION := "0.9.72"
# renovate: datasource=crate depName=sd packageName=sd
SD_VERSION := "1.0.0"
# renovate: datasource=crate depName=cargo-llvm-cov packageName=cargo-llvm-cov
CARGO_LLVM_COV_VERSION := "0.6.13"
# renovate: datasource=crate depName=cargo-hack packageName=cargo-hack
CARGO_HACK_VERSION := "0.6.33"

WASI_TARGET_NAME := "wasm32-wasip2"

@default: build-language-plugin regenerator install-requirements build-wasm-components build-wasm-components-release test-all rust-docs fmt

# Regenerate "DO NOT EDIT" sections, recreate generator examples (but does not compile them), reformat whole project
housekeeping-ci-flow: regenerator regenerate-generator-tests fmt

# Runs all amd64 unit and doc tests tests
base-ci-flow: test

# Runs all examples/*
examples-ci-flow: build-language-plugin build-wasm-components build-wasm-components-release test-examples

# Regenerates provider from generator's integration test
generator-ci-flow COMPILATION_NAME:
    just test-provider-compilation {{COMPILATION_NAME}}

cpp-ci-flow: build-language-plugin build-static-library test-cpp

native-ci-flow: build-language-plugin build-native-examples test-native

# Test docs examples and creates docs
test-docs-ci-flow: test-docs

# https://stackoverflow.com/questions/74524817/why-is-anyhow-not-working-in-the-stable-version
fix-issues:
    cargo check

build-language-plugin:
    cd pulumi-language-gestalt && just

package-language-plugin VERSION:
    cd pulumi-language-gestalt && just package-language-plugin-all {{VERSION}}

install-requirements:
    rustup component add rustfmt
    rustup component add llvm-tools-preview
    cargo binstall --no-confirm cargo-nextest@{{NEXTEST_VERSION}}
    cargo binstall --no-confirm sd@{{SD_VERSION}}
    cargo binstall --no-confirm cargo-llvm-cov@{{CARGO_LLVM_COV_VERSION}}
    cargo binstall --no-confirm cargo-hack@{{CARGO_HACK_VERSION}}

build-native-examples:
    cargo build -p pulumi_gestalt_example_native

# Compiling everything together causes linking issues
build-wasm-components:
    cargo build -p pulumi_gestalt_wasm_runner
    cargo build -p pulumi_gestalt --target={{WASI_TARGET_NAME}}
    cargo build -p pulumi_gestalt_example_simple --target={{WASI_TARGET_NAME}}
    cargo build -p pulumi_gestalt_example_docker --target={{WASI_TARGET_NAME}}
    cargo build -p pulumi_gestalt_example_dependencies --target={{WASI_TARGET_NAME}}
    cargo build -p pulumi_gestalt_example_multiple_providers --target={{WASI_TARGET_NAME}}
    cargo build -p pulumi_gestalt_example_plugins --target={{WASI_TARGET_NAME}}
    cargo build -p pulumi_gestalt_example_secret --target={{WASI_TARGET_NAME}}

build-wasm-components-release:
    cargo build -p pulumi_gestalt_wasm_runner --release
    cargo build -p pulumi_gestalt --target={{WASI_TARGET_NAME}} --release
    cargo build -p pulumi_gestalt_example_simple --target={{WASI_TARGET_NAME}} --release
    cargo build -p pulumi_gestalt_example_docker --target={{WASI_TARGET_NAME}} --release
    cargo build -p pulumi_gestalt_example_dependencies --target={{WASI_TARGET_NAME}} --release
    cargo build -p pulumi_gestalt_example_multiple_providers --target={{WASI_TARGET_NAME}} --release
    cargo build -p pulumi_gestalt_example_plugins --target={{WASI_TARGET_NAME}} --release
    cargo build -p pulumi_gestalt_example_secret --target={{WASI_TARGET_NAME}} --release

build-static-library:
    cargo build -p pulumi_native_c

check:
    cargo fmt -- --check

fmt:
    cd pulumi-language-gestalt && just fmt
    cargo fmt

fmt-clippy:
    cargo clippy --tests --all-features --fix --allow-dirty --allow-staged
    just fmt

clippy:
    cargo clippy --tests --all-features

clippy-to-file:
    cargo clippy --tests --all-features --message-format=json | clippy-sarif | tee rust-clippy-results.sarif | sarif-fmt

regenerator:
    cargo run -p regenerator

regenerate-generator-tests $DO_NOT_COMPILE="true":
    cargo nextest run -p pulumi_gestalt_generator --all-features --test '*' --profile all_cores

publish:
    cargo hack publish -p pulumi_gestalt_wit --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_proto --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_core --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_rust_common --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_grpc_connection --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_rust_adapter --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_rust_adapter_wasm --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_rust_integration --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_rust_adapter_native --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_rust --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_generator --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_build --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_wasm_component_creator --all-features --no-dev-deps --allow-dirty
    cargo hack publish -p pulumi_gestalt_wasm_runner --all-features --no-dev-deps --allow-dirty

test-provider-compilation COMPILATION_NAME:
    cargo llvm-cov nextest -p pulumi_gestalt_generator --cobertura --output-path covertura.xml --features generator_{{COMPILATION_NAME}} --test '*'

test-examples:
    cargo llvm-cov nextest \
        -p pulumi_gestalt_example_simple \
        -p pulumi_gestalt_example_docker \
        -p pulumi_gestalt_example_dependencies \
        -p pulumi_gestalt_example_multiple_providers \
        -p pulumi_gestalt_example_typesystem \
        -p pulumi_gestalt_example_plugins \
        -p pulumi_gestalt_example_secret \
        --cobertura --output-path covertura.xml --features example_test

test-cpp:
    cargo llvm-cov nextest \
        -p pulumi_gestalt_example_cpp \
        --cobertura --output-path covertura.xml --features example_test

test-native:
    cargo llvm-cov nextest \
        -p pulumi_gestalt_example_native \
        --cobertura --output-path covertura.xml --features example_test

generator-tests:
    cargo nextest run --all-features -p pulumi_gestalt_generator

generator-tests-release:
    cargo nextest run --all-features -p pulumi_gestalt_generator --release

test-all:
    cargo llvm-cov nextest --cobertura --output-path covertura.xml --all-features

test-all-release:
    cargo llvm-cov nextest --cobertura --output-path covertura.xml --all-features --release

test:
    cargo llvm-cov nextest --cobertura --output-path covertura.xml

docs:
    docker run --rm -it -p 8000:8000 -v ${PWD}:/docs squidfunk/mkdocs-material:9.6.4

test-docs:
    cargo test --doc
    just rust-docs
    just rust-docs-wasm

rust-docs:
    cargo doc --no-deps \
        -p pulumi_gestalt_rust \
        -p pulumi_gestalt_build \
        -p pulumi_gestalt_rust_adapter \
        -p pulumi_gestalt_rust_adapter_native \
        -p pulumi_gestalt_rust_integration \
        -p pulumi_gestalt_providers_aws_mini \
        -p pulumi_gestalt_providers_azure_mini \
        -p pulumi_gestalt_providers_cloudflare \
        -p pulumi_gestalt_providers_docker \
        -p pulumi_gestalt_providers_gcp_mini \
        -p pulumi_gestalt_providers_random

rust-docs-wasm:
    cargo doc --no-deps --target wasm32-wasip2 \
        -p pulumi_gestalt_rust \
        -p pulumi_gestalt_build \
        -p pulumi_gestalt_rust_adapter \
        -p pulumi_gestalt_rust_adapter_wasm \
        -p pulumi_gestalt_providers_aws_mini \
        -p pulumi_gestalt_providers_azure_mini \
        -p pulumi_gestalt_providers_cloudflare \
        -p pulumi_gestalt_providers_docker \
        -p pulumi_gestalt_providers_gcp_mini \
        -p pulumi_gestalt_providers_random

rust-docs-release $RUSTDOCFLAGS="--html-in-header docs_additions/umami.html":
    just rust-docs

rust-docs-wasm-release $RUSTDOCFLAGS="--html-in-header docs_additions/umami.html":
    just rust-docs-wasm

update-version NEW_VERSION:
    sd "0.0.0-DEV" "{{NEW_VERSION}}" "crates/wit/wit/world.wit" "crates/rust/src/lib.rs" \
    "Cargo.toml"
