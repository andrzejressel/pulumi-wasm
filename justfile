set windows-shell := ["pwsh.exe", "-c"]
# renovate: datasource=crate depName=cargo-nextest packageName=cargo-nextest
NEXTEST_VERSION := "0.9.72"
# renovate: datasource=crate depName=cargo-component packageName=cargo-component
CARGO_COMPONENT_VERSION := "0.13.0"
# renovate: datasource=crate depName=wasm-tools packageName=wasm-tools
WASM_TOOLS_VERSION := "1.202.0"

@default: build test

build: build-language-plugin regenerate-providers install-requirements build-wasm-components fmt

# https://stackoverflow.com/questions/74524817/why-is-anyhow-not-working-in-the-stable-version
fix-issues:
    cargo component check --workspace
    cargo check --workspace

build-language-plugin:
    cd pulumi-language-wasm && just

install-requirements:
    cargo binstall cargo-nextest@{{NEXTEST_VERSION}} --locked || cargo-nextest --version
    cargo binstall cargo-component@{{CARGO_COMPONENT_VERSION}} --locked || cargo-component --version
    cargo binstall wasm-tools@{{WASM_TOOLS_VERSION}} --locked || wasm-tools --version

build-wasm-components:
    cargo component build -p pulumi_wasm \
                          -p pulumi_wasm_example_simple \
                          -p pulumi_wasm_example_docker
    # DO NOT EDIT - BUILD-WASM-COMPONENTS - START
    cargo component build \
      -p pulumi_wasm_docker_provider \
      -p pulumi_wasm_random_provider \
    # DO NOT EDIT - BUILD-WASM-COMPONENTS - END
    cargo run -p cargo-pulumi -- -p pulumi_wasm_example_dependencies
    cargo run -p cargo-pulumi -- -p pulumi_wasm_example_docker
    cargo run -p cargo-pulumi -- -p pulumi_wasm_example_simple
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
    cargo run -p cargo-pulumi-gen -- gen-provider --remove true --schema providers/docker.json --output providers/pulumi_wasm_provider_docker
    cargo run -p cargo-pulumi-gen -- gen-rust     --remove true --schema providers/docker.json --output providers/pulumi_wasm_provider_docker_rust
    cargo run -p cargo-pulumi-gen -- gen-provider --remove true --schema providers/random.json --output providers/pulumi_wasm_provider_random
    cargo run -p cargo-pulumi-gen -- gen-rust     --remove true --schema providers/random.json --output providers/pulumi_wasm_provider_random_rust
# DO NOT EDIT - REGENERATE-PROVIDERS - END

test:
    cargo nextest run --workspace

docs:
    docker run --rm -it -p 8000:8000 -v ${PWD}:/docs squidfunk/mkdocs-material
