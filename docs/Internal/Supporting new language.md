# Checklist for new language

New language should have the following features:

- `component:pulumi-gestalt/pulumi-main.main` [Entrypoint](WIT.md#pulumi-main)
- Abstration over `Output<T>` (mapping, combining)
- Function map loop described in [Output](Output.md/#mapping)
- Generating glue code for providers

`pulumi_gestalt_rust` and `pulumi_gestalt_macro` should be used as an example - combined they have about 100 LOC and they
only create abstration over Wasm calls.

## Optional

### Saving provider metadata in binary

When generating glue code for providers, it is possible to save provider metadata in binary. This is done by adding custom section to binary.

```text
(@custom "pulumi_gestalt_provider::random" (after data) "{\"version\":\"4.15.0\",\"pluginDownloadURL\":null}")
(@custom "pulumi_gestalt_provider::docker" (after data) "{\"version\":\"4.5.3\",\"pluginDownloadURL\":null}")
```

In Rust, it looks like this:

```rust
#[link_section = "pulumi_gestalt_provider::docker"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub static PULUMI_WASM_PROVIDER_DOCKER: [u8; 44] = *b"{\"version\":\"4.5.3\",\"pluginDownloadURL\":null}";
```

Key is `pulumi_gestalt_provider::<provider_name>` and value is JSON with required `version` and optional `pluginDownloadURL` fields.
Both of these keys are available in the provider's schema.
