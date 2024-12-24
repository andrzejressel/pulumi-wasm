set windows-shell := ["pwsh.exe", "-c"]
# renovate: datasource=crate depName=cargo-nextest packageName=cargo-nextest
NEXTEST_VERSION := "0.9.72"
# renovate: datasource=crate depName=cargo-component packageName=cargo-component
CARGO_COMPONENT_VERSION := "0.19.0"
# renovate: datasource=crate depName=sd packageName=sd
SD_VERSION := "1.0.0"
# renovate: datasource=crate depName=cargo-llvm-cov packageName=cargo-llvm-cov
CARGO_LLVM_COV_VERSION := "0.6.13"

FORMATTABLE_PROJECTS := "-p pulumi_wasm -p pulumi_wasm_common -p pulumi_wasm_generator -p pulumi_wasm_generator_lib \
-p pulumi_wasm_runner -p pulumi_wasm_runner_component_creator -p pulumi_wasm_rust -p pulumi_wasm_rust_macro \
-p pulumi_wasm_example_dependencies -p pulumi_wasm_example_docker -p pulumi_wasm_example_multiple_providers \
-p pulumi_wasm_example_simple -p pulumi_wasm_example_typesystem -p regenerate_providers"

@default: build test

build: build-language-plugin install-requirements build-wasm-components build-all-wasm-projects-release fmt

# https://stackoverflow.com/questions/74524817/why-is-anyhow-not-working-in-the-stable-version
fix-issues:
    cargo component check --workspace
    cargo check --workspace

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

# Compiling everything together causes linking issues
build-wasm-components:
    cargo build -p pulumi_wasm_runner
    cargo component build -p pulumi_wasm
    cargo component build -p pulumi_wasm_example_simple
    cargo component build -p pulumi_wasm_example_docker
    cargo component build -p pulumi_wasm_example_dependencies
    cargo component build -p pulumi_wasm_example_multiple_providers

build-all-wasm-projects-release:
    cargo build -p pulumi_wasm_runner --release
    cargo component build -p pulumi_wasm --release
    cargo component build -p pulumi_wasm_example_simple --release
    cargo component build -p pulumi_wasm_example_docker --release
    cargo component build -p pulumi_wasm_example_dependencies --release
    cargo component build -p pulumi_wasm_example_multiple_providers --release

build-wasm-components-release:

check:
    cargo fmt {{FORMATTABLE_PROJECTS}} -- --check

fmt:
    cd pulumi-language-wasm && just fmt
    cargo fmt {{FORMATTABLE_PROJECTS}}

fmt-clippy:
    cargo clippy --all-features --fix --allow-dirty --allow-staged {{FORMATTABLE_PROJECTS}}
    just fmt

clippy-to-file:
    cargo clippy --all-features --message-format=json {{FORMATTABLE_PROJECTS}} | clippy-sarif | tee rust-clippy-results.sarif | sarif-fmt

regenerator:
    cargo run -p regenerate_providers

publish:
    cargo publish -p pulumi_wasm_wit --all-features
    cargo publish -p pulumi_wasm_proto --all-features
    cargo publish -p pulumi_wasm_common --all-features
    cargo publish -p pulumi_wasm_rust_macro --all-features
    cargo publish -p pulumi_wasm_rust --all-features
    cargo publish -p pulumi_wasm_generator_lib --all-features
    cargo publish -p pulumi_wasm_generator --all-features
    cargo publish -p pulumi_wasm_core --all-features
    cargo publish -p pulumi_wasm_runner_component_creator --all-features
    cargo publish -p pulumi_wasm_runner --all-features
    cargo publish -p pulumi_wasm_provider_common --all-features

test:
    cargo nextest run --profile ci --workspace --timings
    just rust-docs

test-coverage:
    cargo llvm-cov --no-report -p pulumi_wasm_core -p pulumi_wasm_generator_lib
    cargo llvm-cov report --lcov --output-path lcov.info

docs:
    docker run --rm -it -p 8000:8000 -v ${PWD}:/docs squidfunk/mkdocs-material

# DO NOT EDIT - GENERATE-RUST-DOCS - START
rust-docs:
    cargo test --doc -p pulumi_wasm_providers_random -p pulumi_wasm_providers_cloudflare
    cargo doc --no-deps -p pulumi_wasm_rust -p pulumi_wasm_providers_docker -p pulumi_wasm_providers_random -p pulumi_wasm_providers_cloudflare
# DO NOT EDIT - GENERATE-RUST-DOCS - END

update-version NEW_VERSION:
    sd "0.0.0-DEV" "{{NEW_VERSION}}" "pulumi_wasm_wit/wit/world.wit" "pulumi_wasm_rust_macro/src/lib.rs" \
    "Cargo.toml"
