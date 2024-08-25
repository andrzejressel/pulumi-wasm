## Pulumi support for WASM

![GitHub Release](https://img.shields.io/github/v/release/andrzejressel/pulumi-wasm?include_prereleases&sort=date)
[![Build](https://github.com/andrzejressel/pulumi-wasm/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/andrzejressel/pulumi-wasm/actions/workflows/build.yml)
[![Deploy](https://github.com/andrzejressel/pulumi-wasm/actions/workflows/deploy.yaml/badge.svg)](https://github.com/andrzejressel/pulumi-wasm/actions/workflows/deploy.yaml)


PoC of Pulumi support for WASM.

### Quick start

https://github.com/andrzejressel/pulumi-wasm-example

### Installation

#### Language plugin

```
pulumi plugin install language wasm "0.0.0-NIGHTLY-SHORTSHA1" --server github://api.github.com/andrzejressel/pulumi-wasm
```

#### WASM Runner

```
cargo binstall -y --index "sparse+https://cargo.cloudsmith.io/andrzej-ressel-github/pulumi-wasm/" pulumi_wasm_runner@0.0.0-NIGHTLY-SHORTSHA1
```

### Acknowledgements

[![Hosted By: Cloudsmith](https://img.shields.io/badge/OSS%20hosting%20by-cloudsmith-blue?logo=cloudsmith&style=for-the-badge)](https://cloudsmith.com)

Package repository hosting is graciously provided by  [Cloudsmith](https://cloudsmith.com).
Cloudsmith is the only fully hosted, cloud-native, universal package management solution, that
enables your organization to create, store and share packages in any format, to any place, with total
confidence.
