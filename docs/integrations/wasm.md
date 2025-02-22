# Wasm

!!! note "Please Read First"
Before proceeding, make sure to read this [Overview](overview.md) page to get a better understanding of the
documentation.

In Pulumi Gestalt, Wasm support is based on the [Component Model](https://component-model.bytecodealliance.org/).

## Artifacts

Several artifacts related to Wasm support can be found on
the [releases](https://github.com/andrzejressel/pulumi-gestalt-releases/releases/) page:

- **Wasm implementation** (`pulumi_gestalt-debug.wasm` / `pulumi_gestalt-release.wasm`)
- **Runner** (`pulumi_gestalt_wasm_runner`)
- **WIT files** (`world.wit`, `pulumi-gestalt-external.wit`)

The key artifacts are the **Runner** and **WIT files**. The Runner automatically downloads the corresponding Wasm
implementation from the releases page when the program is executed.

## Versioning

Currently, WIT files follow nightly versioning. Integrations should select a nightly version of Pulumi Gestalt to ensure
compatibility. The Runner downloads the corresponding Wasm implementation and merges them into a single Wasm file.

## Entrypoint

The entry point is managed by the `component:pulumi-gestalt-external/pulumi-main` interface. This function is invoked by
`pulumi-gestalt-runner`. It takes a single argument, `in-preview`, which should be passed to
the context constructor.

## Callback Emulation

```TODO```

## Runner Quick Start

To execute a Pulumi Gestalt Wasm program using the Runner, use:

```sh
pulumi_wasm_runner run <WASM_FILE>
```

This downloads the corresponding Wasm implementation, merges it with the specified Wasm file, and runs the program. To
use the debug version of Wasm, add the `--debug` flag.

## WIT Files

```title="world.wit"
--8<-- "crates/wit/wit/world.wit"
```

```title="pulumi-gestalt-external.wit"
--8<-- "crates/wit/wit/deps/pulumi-gestalt-external.wit"
```

The relevant integration world for Pulumi Gestalt is `client`.
