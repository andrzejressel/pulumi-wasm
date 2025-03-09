# Overview

Pulumi Gestalt is made to support any language. Due to that abstractions over running code must be as small as possible - 
actual program is run using `just` recipes.

## Pulumi Gestalt Language plugin

Pulumi Gestalt uses a custom language plugin that can be installed using Pulumi CLI:

```shell
pulumi plugin install language gestalt "VERSION" --server github://api.github.com/andrzejressel/pulumi-gestalt
```

Currently there are no configuration options for the plugin. It can be used in following way:

```yaml title="Pulumi.yaml"
name: Some_name
runtime: gestalt
```

The plugin will run `just` recipies only when `Justfile` is located in the same directory as the Pulumi program. On other case 
the error will be thrown.

### Just

[Just](https://github.com/casey/just) is `Make` alternative used by Pulumi Gestalt to invoke programs in cross-platform
and very generic way.

Currently, there are two recipes that are used by Pulumi Gestalt:

- `run`: used to run program
- `plugins TEMP_DIRECTORY` (optional): used to get list of used plugins. File with a JSON serialized list of plugins is expected in TEMP_DIRECTORY.

Example recipes:

#### Rust (native)

```just title="Justfile"
run:
    cargo run
```

#### Rust (Wasm)

```just title="Justfile"
binary := "pulumi_gestalt_wasm_runner" # Runner described in Rust language section
wasm := "../target/wasm32-wasip2/debug/pulumi_gestalt_example_wasm.wasm"

run:
    cargo build --target wasm32-wasip2
    {{binary}} run --debug "{{wasm}}"
```

#### C++

```just title="Justfile"
set windows-shell := ["pwsh.exe", "-c"]

[windows]
run:
    New-Item -ItemType Directory -Path build -Force
    cd build && cmake .. && cmake --build .
    .\build\Debug\executable.exe

[unix]
run:
    mkdir -p build
    cd build && cmake .. && cmake --build .
    ./build/executable
```