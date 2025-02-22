# Overview

**Pulumi Gestalt** is a framework that simplifies adding new language support to the Pulumi ecosystem. It provides APIs
for **WebAssembly (Wasm)**, **C FFI (Foreign Function Interface)**, and **Rust**, enabling seamless integration of new
languages with Pulumi.

## Motivation

Pulumi currently supports a limited number of programming languages. Adding support for a new language typically
requires significant effort to bridge the language with Pulumi's core infrastructure. Pulumi Gestalt aims to reduce this
effort by providing a common set of tools and APIs for language integration.

The framework is designed to work with high-level and low-level languages, allowing developers to focus on
language-specific integration details without worrying about the underlying Pulumi details.

## User Guide

- [Rust](languages/rust/index.md)

## Integration Guide

- [C FFI](integrations/c-ffi.md)
- [Rust](integrations/rust.md)
- [Wasm](integrations/wasm.md)