## Pulumi Gestalt

![GitHub Release](https://img.shields.io/github/v/release/andrzejressel/pulumi-gestalt-releases?include_prereleases&sort=date)
[![Build](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/build.yml)
[![Deploy](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/deploy.yml/badge.svg)](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/deploy.yml)
[![Docs](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/docs.yml/badge.svg)](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/docs.yml)
[![codecov](https://codecov.io/gh/andrzejressel/pulumi-gestalt/graph/badge.svg?token=J3IN76CSOP)](https://codecov.io/gh/andrzejressel/pulumi-gestalt)


Pulumi support for **any** language

### Quick start

https://github.com/andrzejressel/pulumi-gestalt-example

### Installation

#### Language plugin

```
pulumi plugin install language gestalt "VERSION" --server github://api.github.com/andrzejressel/pulumi-gestalt-releases
```

#### Wasm Runner

```
cargo binstall -y --index "sparse+https://cargo.cloudsmith.io/andrzej-ressel-github/pulumi-gestalt/" pulumi_gestalt_wasm_runner@VERSION
```

### Acknowledgements

[![Hosted By: Cloudsmith](https://img.shields.io/badge/OSS%20hosting%20by-cloudsmith-blue?logo=cloudsmith&style=for-the-badge)](https://cloudsmith.com)

Package repository hosting is graciously provided by [Cloudsmith](https://cloudsmith.com).
Cloudsmith is the only fully hosted, cloud-native, universal package management solution, that
enables your organization to create, store and share packages in any format, to any place, with total
confidence.
