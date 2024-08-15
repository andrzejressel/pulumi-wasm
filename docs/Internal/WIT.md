# WIT

``` title="world.wit"
--8<-- "pulumi_wasm_wit/wit/world.wit"
```

## Interfaces

### Output

Allow working with Outputs (so maybe not yet set values managed by Pulumi). Interface allows creating, mapping and cloning (to be removed in the future).
It also allows to combine list of outputs into output of list

### Stack

(Name to be changed). Currently, this interface allows adding output as export ([Pulumi docs](https://www.pulumi.com/tutorials/building-with-pulumi/stack-outputs/)).
And invoking `finish` function - explained in [Output](Output.md/#mapping).

### Pulumi main

Entrypoint to program that will be invoked by `pulumi-wasm-runner`.
In Rust case it is created by `pulumi_wasm` macro.

### Log

Allows logging from within provider (used mostly for main pulumi-wasm component)

### Register

Interface that allows registering of Pulumi resources. Used only by providers.

### External world

Interface used as a substitute until WASM gets proper GRPC client support. 
It is used by main `pulumi-wasm` component binary and is implemented by `pulumi-wasm-runner`.

## Worlds

### client world

World used by client languages. Imports output and stack interfaces and exports pulumi-main interface.

### pulumi-wasm world

World used by main pulumi-wasm component. A

### logger world

World that only contains `log` interface.