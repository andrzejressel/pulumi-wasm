# Crates

### Pulumi WASM

Main WASM component. Currently, implements Output handling, send and handles requests to Pulumi.

### Pulumi WASM runner

x64 application that combines and executes Pulumi WASM components.

### Pulumi WASM Rust

Rust library that provides a high-level and typesafe API for Pulumi WASM. It's a wrapper around `pulumi-wasm` interfaces.
Also provides `pulumi_main` macro via `pub use` from `Pulumi WASM Rust Macro`.

### Pulumi WASM Rust Macro

Rust library with `pulumi_main` macro. Addon to `Pulumi WASM Rust`

### Pulumi WASM Common

Library used in WASM components of Pulumi providers. Currently provides logging facilities.
