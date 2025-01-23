# Checklist for new language

New language should have the following features:

- `component:pulumi-wasm/pulumi-main.main` [Entrypoint](WIT.md#pulumi-main)
- Abstration over `Output<T>` (mapping, combining)
- Function map loop described in [Output](Output.md/#mapping)
- Generating glue code for providers

`pulumi_wasm_rust` and `pulumi_wasm_macro` should be used as an example - combined they have about 100 LOC and they
only create abstration over Wasm calls.

## Optional

### Saving provider metadata in binary

When generating glue code for providers, it is possible to save provider metadata in binary. This is done by adding custom section to binary.

```text
(@custom "pulumi_wasm_provider::random" (after data) "{\22version\22:\224.15.0\22,\22pluginDownloadURL\22:null}")
(@custom "pulumi_wasm_provider::docker" (after data) "{\22version\22:\224.5.3\22,\22pluginDownloadURL\22:null}")
```

In Rust, it looks like this:

```rust
#[link_section = "pulumi_wasm_provider::docker"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub static PULUMI_Wasm_PROVIDER_DOCKER: [u8; 44] = *b"{\"version\":\"4.5.3\",\"pluginDownloadURL\":null}";
```

Key is `pulumi_wasm_provider::<provider_name>` and value is JSON with required `version` and optional `pluginDownloadURL` fields.
Both of these keys are available in the provider's schema.