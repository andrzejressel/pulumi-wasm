# C FFI

!!! note "Please Read First"
    Before proceeding, make sure to read this [Pulumi Gestalt integrations Overview](overview.md) page to get a better understanding of the documentation.

## Artifacts

Shared library and header file are available on the [release page](https://github.com/andrzejressel/pulumi-gestalt-releases/releases)

## Header file 

```cpp title="pulumi_gestalt.h"
--8<-- "crates/c_ffi/pulumi_gestalt.h"
```

## Cleanup

`Output` and `CompositeOutput` does not need to be explicitly cleaned up. They are automatically managed by the SDK and will be destroyed when the context is destroyed. 