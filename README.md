## Pulumi Gestalt

![GitHub Release](https://img.shields.io/github/v/release/andrzejressel/pulumi-gestalt?include_prereleases&sort=date)
[![Build](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/build.yml)
[![Deploy](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/deploy.yml/badge.svg)](https://github.com/andrzejressel/pulumi-gestalt/actions/workflows/deploy.yml)
[![Docs](https://readthedocs.org/projects/pulumi-gestalt/badge/?version=latest)](https://app.readthedocs.org/projects/pulumi-gestalt/builds/?version__slug=latest)
[![codecov](https://codecov.io/gh/andrzejressel/pulumi-gestalt/graph/badge.svg?token=J3IN76CSOP)](https://codecov.io/gh/andrzejressel/pulumi-gestalt)


Pulumi support for **any** language

### Quick start

https://github.com/andrzejressel/pulumi-gestalt-example

### Installation

#### Language plugin

```
pulumi plugin install language gestalt "VERSION" --server github://api.github.com/andrzejressel/pulumi-gestalt
```

#### Wasm Runner

```
cargo binstall -y pulumi_gestalt_wasm_runner@VERSION
```