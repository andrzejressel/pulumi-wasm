# Checklist for new language

New language should have the following features:

- `component:pulumi-wasm/pulumi-main.main` [Entrypoint](WIT.md#pulumi-main)
- Abstration over `Output<T>` (mapping, combining)
- Function map loop described in [Output](Output.md/#mapping)
- Generating glue code for providers

`pulumi_wasm_rust` and `pulumi_wasm_macro` should be used as an example - combined they have about 100 LOC and they
only create abstration over WASM calls.