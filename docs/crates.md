# Crates

### Pulumi Wasm

Main Wasm component. Currently, implements Output handling, send and handles requests to Pulumi.

### Pulumi Wasm runner

x64 application that combines and executes Pulumi Wasm components.

### Pulumi Wasm Rust

Rust library that provides a high-level and typesafe API for Pulumi Wasm. It's a wrapper around `pulumi-gestalt` interfaces.
Also provides `pulumi_main` macro via `pub use` from `Pulumi Wasm Rust Macro`.

### Pulumi Wasm Rust Macro

Rust library with `pulumi_main` macro. Addon to `Pulumi Wasm Rust`

### Pulumi Wasm Common

Library used in Wasm components of Pulumi providers. Currently provides logging facilities.
