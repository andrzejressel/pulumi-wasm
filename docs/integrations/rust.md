# Rust

!!! note "Please Read First"
    Before proceeding, make sure to read this [Overview](overview.md) page to get a better understanding of the documentation.


Rust integration is available as `pulumi_gestalt_rust_integration` package in project's custom crate repository.

You can setup this repository using

```toml title=".cargo/config.toml"
[registries.pulumi-gestalt]
index = "sparse+https://cargo.cloudsmith.io/andrzej-ressel-github/pulumi-gestalt/"
```

and dependency can be then added using

```toml title="Cargo.toml"
[dependencies]
pulumi_gestalt_rust_integration = {version = "=<PULUMI_GESTALT_VERSION>", registry = "pulumi-gestalt"}
```

rustdoc for integration is available [here](https://andrzejressel.github.io/pulumi-gestalt/rust-docs/pulumi_gestalt_rust_integration/index.html)