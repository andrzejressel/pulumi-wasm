# Wasm

!!! note "Please Read First"
    Before proceeding, make sure to read this [Overview](overview.md) page to get a better understanding of the documentation.

In Pulumi Gestalt Wasm support is based on [Component Model](https://component-model.bytecodealliance.org/)

## Artifacts

There are multiple artifacts related to Wasm support on the [releases](https://github.com/andrzejressel/pulumi-gestalt-releases/releases/) page:

- Wasm implementation (`pulumi_gestalt-debug.wasm`/`pulumi_gestalt-release.wasm`)
- Runner (`pulumi_gestalt_wasm_runner`)
- WIT files (`world.wit`, `pulumi-gestalt-external.wit`)

The most important artifacts are Runner and WIT files. 
Runner will automatically download corresponding wasm implementation from releases page when program is run.

## Versioning

Currently, WIT files are versioned with nightly versions. That means that integration should select a nightly version of Pulumi Gestalt that it wants to integrate with.
Runner will automatically download the corresponding Wasm implementation and merge them to create a single Wasm file.

## Entrypoint

Entrypoint is managed by `component:pulumi-gestalt-external/pulumi-main` interface. It is a function that is called by `pulumi-gestalt-runner` and is responsible for executing the program.
It contains single argument `in-preview` which should be passed to context constructor.

## Callback emulation



## Runner quick start

The simplest to run Pulumi Gestalt Wasm program with runner would be:

```
pulumi_wasm_runner run <WASM_FILE>
```

It will download corresponding Wasm implementation, merge it with Wasm file and run the program.
If debug version of Wasm is needed, `--debug` flag can be passed.


## WIT files

``` title="world.wit"
--8<-- "crates/wit/wit/world.wit"
```

``` title="pulumi-gestalt-external.wit"
--8<-- "crates/wit/wit/deps/pulumi-gestalt-external.wit"
```


World that is relevant for integration with Pulumi Gestalt is `client`