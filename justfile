set windows-shell := ["pwsh.exe", "-c"]
# renovate: datasource=crate depName=cargo-nextest packageName=cargo-nextest
NEXTEST_VERSION := "0.9.72"
# renovate: datasource=crate depName=cargo-component packageName=cargo-component
CARGO_COMPONENT_VERSION := "0.14.0"
# renovate: datasource=crate depName=sd packageName=sd
SD_VERSION := "1.0.0"

@default: build test

build: build-language-plugin regenerate-providers install-requirements build-wasm-components fmt

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
    cargo binstall --no-confirm cargo-nextest@{{NEXTEST_VERSION}}
    cargo binstall --no-confirm cargo-component@{{CARGO_COMPONENT_VERSION}}
    cargo binstall --no-confirm sd@{{SD_VERSION}}

# Compiling everything together causes linking issues
build-wasm-components:
    cargo component build -p pulumi_wasm --timings
    cargo component build -p pulumi_wasm_example_simple --timings
    cargo component build -p pulumi_wasm_example_docker --timings
    cargo component build -p pulumi_wasm_example_dependencies --timings
    cargo component build -p pulumi_wasm_example_multiple_providers --timings
    # DO NOT EDIT - BUILD-WASM-COMPONENTS - START
    cargo component build \
      -p pulumi_wasm_docker_provider \
      -p pulumi_wasm_random_provider \
      --timings
    # DO NOT EDIT - BUILD-WASM-COMPONENTS - END
    cargo build -p pulumi_wasm_runner --timings

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

publish:
    cargo publish -p pulumi_wasm_wit --allow-dirty --all-features
    cargo publish -p pulumi_wasm_proto --allow-dirty --all-features
    cargo publish -p pulumi_wasm_common --allow-dirty --all-features
    cargo publish -p pulumi_wasm_rust_macro --allow-dirty --all-features
    cargo publish -p pulumi_wasm_rust --allow-dirty --all-features
    cargo publish -p pulumi_wasm_generator_lib --allow-dirty --all-features
    cargo publish -p pulumi_wasm_generator --allow-dirty --all-features
    cargo publish -p pulumi_wasm_core --allow-dirty --all-features
    cargo publish -p pulumi_wasm_docker --allow-dirty --all-features
    cargo publish -p pulumi_wasm_random --allow-dirty --all-features
    cargo publish -p pulumi_wasm_runner --allow-dirty --all-features

test:
    cargo nextest run --workspace --timings

docs:
    docker run --rm -it -p 8000:8000 -v ${PWD}:/docs squidfunk/mkdocs-material

update-version NEW_VERSION:
    sd "0.0.0-DEV" "{{NEW_VERSION}}" "pulumi_wasm_wit/wit/world.wit" "pulumi_wasm_rust_macro/src/lib.rs" \
    "providers/pulumi_wasm_provider_docker_rust/Cargo.toml" \
    "providers/pulumi_wasm_provider_random_rust/Cargo.toml" \
    "Cargo.toml"
