## Pulumi support for WASM

PoC of Pulumi support for WASM.

Currently, supports very simple code for very simple providers (no complex objects). 

### Installation

#### Language plugin

```
pulumi plugin install language wasm "0.1.0-SHORTSHA1" --server github://api.github.com/andrzejressel/pulumi-wasm
```