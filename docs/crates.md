# Crates

### Pulumi WASM

Main WASM component. Currently implements Output handling, send and handles requests to Pulumi.

### Pulumi WASM runner

x64 application that runs the WASM component. 
Implements `component:pulumi-wasm@0.0.0/external-world` and `component:pulumi-wasm@0.0.0/log` interfaces.

### Pulumi WASM Rust

Rust library that provides a high-level and typesafe API for Pulumi WASM. It's a wrapper around `pulumi-wasm` interfaces.
Also provides `pulumi_main` macro via `pub use` from `Pulumi WASM Rust Macro`.

### Pulumi WASM Rust Macro

Rust library with `pulumi_main` macro. Addon to `Pulumi WASM Rust`

### WASM Common

Library used in WASM components of Pulumi providers. Currently provides logging facilities.

### Pulumi WASM Provider Random

WASM component for Pulumi's Random provider. Currently handwritten - 
after [#5](https://github.com/andrzejressel/pulumi-wasm/issues/5) generated.

### Pulumi WASM Provider Random Rust

Rust library that provides a high-level and typesafe API for `Pulumi WASM Provider Random` WASM component.
Currently handwritten - after [#5](https://github.com/andrzejressel/pulumi-wasm/issues/5) generated.

### cargo pulumi

Cargo subcommands that compile and combine Pulumi WASM components into a single WASM file.

### examples/simple

Currently the only example. It's a simple Pulumi program that uses `Pulumi WASM Provider Random Rust` to generate random numbers.
In future will be one of integration tests.