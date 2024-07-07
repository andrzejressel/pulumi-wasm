set windows-shell := ["pwsh.exe", "-c"]
# renovate: datasource=crate depName=cargo-nextest packageName=cargo-nextest
NEXTEST_VERSION := "0.9.72"
# renovate: datasource=crate depName=cargo-component packageName=cargo-component
CARGO_COMPONENT_VERSION := "0.13.0"

@default: build test

build: build-language-plugin regenerate-providers install-requirements build-wasm-components fmt

# https://stackoverflow.com/questions/74524817/why-is-anyhow-not-working-in-the-stable-version
fix-issues:
    cargo component check --workspace
    cargo check --workspace

build-language-plugin:
    cd pulumi-language-wasm && just

install-requirements:
    rustup component add rustfmt
    cargo binstall --no-confirm cargo-nextest@{{NEXTEST_VERSION}} --force || cargo-nextest --version
    cargo binstall --no-confirm cargo-component@{{CARGO_COMPONENT_VERSION}} --force || cargo-component --version

build-wasm-components:
    cargo component build -p pulumi_wasm \
                          -p pulumi_wasm_example_simple \
                          -p pulumi_wasm_example_docker \
                          -p pulumi_wasm_example_dependencies \
                          -p pulumi_wasm_example_multiple_providers
    # DO NOT EDIT - BUILD-WASM-COMPONENTS - START
    cargo component build \
      -p pulumi_wasm_docker_provider \
      -p pulumi_wasm_random_provider \
    # DO NOT EDIT - BUILD-WASM-COMPONENTS - END
    cargo build -p pulumi_wasm_runner
    cargo test --no-run --all

check:
    cargo fmt --all -- --check

fmt:
    cd pulumi-language-wasm && just fmt
    cargo fmt --all

fmt-clippy:
    cargo clippy --all --all-features --fix --allow-dirty --allow-staged
    just fmt

regenerate-provider-list:
    cargo run -p regenerate_providers

# DO NOT EDIT - REGENERATE-PROVIDERS - START
regenerate-providers:
    cargo run -p pulumi_wasm_generator -- gen-provider --remove true --schema providers/docker.json --output providers/pulumi_wasm_provider_docker
    cargo run -p pulumi_wasm_generator -- gen-rust     --remove true --schema providers/docker.json --output providers/pulumi_wasm_provider_docker_rust
    cargo run -p pulumi_wasm_generator -- gen-provider --remove true --schema providers/random.json --output providers/pulumi_wasm_provider_random
    cargo run -p pulumi_wasm_generator -- gen-rust     --remove true --schema providers/random.json --output providers/pulumi_wasm_provider_random_rust
# DO NOT EDIT - REGENERATE-PROVIDERS - END

test:
    cargo nextest run --workspace

docs:
    docker run --rm -it -p 8000:8000 -v ${PWD}:/docs squidfunk/mkdocs-material
