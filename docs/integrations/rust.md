# Rust

!!! note "Please Read First"
    Before proceeding, make sure to read [Pulumi Gestalt integrations Overview](overview.md) page to get a better understanding of the documentation.


Rust integration is available as `pulumi_gestalt_rust_integration` crate.

```toml title="Cargo.toml"
[dependencies]
pulumi_gestalt_rust_integration = { version = "=<PULUMI_GESTALT_VERSION>" }
```

rustdoc is available on [docs.rs](https://docs.rs/pulumi_gestalt_rust_integration/latest/pulumi_gestalt_rust_integration/)

## Example

Here is an example of a program that creates, invokes resources, and uses output operations.

```rust title="main.rs"
--8<-- "examples/raw_rust/src/main.rs"
```
