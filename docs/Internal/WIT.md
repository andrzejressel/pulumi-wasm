# WIT

``` title="world.wit"
--8<-- "crates/wit/wit/world.wit"
```

``` title="pulumi-gestalt-external.wit"
--8<-- "crates/wit/wit/deps/pulumi-gestalt-external.wit"
```

## Interfaces

### Output

Allow working with Outputs (so maybe not yet set values managed by Pulumi). Interface allows creating and mapping underlying values.
It also allows to combine list of outputs into output of list

### Stack

(Name to be changed). Currently, this interface allows adding output as export ([Pulumi docs](https://www.pulumi.com/tutorials/building-with-pulumi/stack-outputs/)).
And invoking `finish` function - explained in [Output](Output.md/#mapping).

### Pulumi main

Entrypoint to program that will be invoked by `pulumi-gestalt-runner`.
In Rust case it is created by `pulumi_gestalt` macro.

### Log

Allows logging from within provider (used mostly for main pulumi-gestalt component)

### Register

Interface that allows registering of Pulumi resources. Used only by providers.

### External world

Interface used as a substitute until Wasm gets proper GRPC client support. 
It is used by main `pulumi-gestalt` component binary and is implemented by `pulumi-gestalt-runner`.

## Worlds

### client world

World used by client languages. Imports output and stack interfaces and exports pulumi-main interface.

### pulumi-gestalt world

World used by main pulumi-gestalt component. A

### logger world

World that only contains `log` interface.