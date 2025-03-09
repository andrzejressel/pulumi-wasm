# Rust

!!! note "Please Read First"
    Before proceeding, make sure to read [Pulumi Gestalt integrations Overview](overview.md) page to get a better understanding of the documentation.


Rust integration is available as `pulumi_gestalt_rust_integration` package in project's custom crate repository.

You can setup this repository using

and dependency can be then added using

```toml title="Cargo.toml"
[dependencies]
pulumi_gestalt_rust_integration = { version = "=<PULUMI_GESTALT_VERSION>" }
```

rustdoc for integration is available [here](https://andrzejressel.github.io/pulumi-gestalt/rust-docs/pulumi_gestalt_rust_integration/index.html)

## Example

Here is an example of a program that creates, invokes resources, and uses output operations.

```rust title="main.rs"
--8<-- "examples/raw_rust/src/main.rs"
```