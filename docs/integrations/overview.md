# Overview

!!! note "Walk around the code first"
    Before processing, feel free to do [Rust tutorial](../languages/rust/index.md), open code in your IDE, walk through it and check abstractions behind `Output` and generated provider.


## Abstractions

The `Pulumi Gestalt` SDK provides abstractions for working with Pulumi in a structured way. It is built around three core concepts:

1. **Context**: Represents a single execution environment for Pulumi operations. It is not recommended to use a global context, 
   as doing so may limit compatibility with advanced features like the [Pulumi Automation API](https://www.pulumi.com/docs/iac/using-pulumi/automation-api/).
2. **Output**: Represents values that are resolved asynchronously (e.g., cloud resource outputs).
3. **CompositeOutput**: Represents a map of outputs returned by a resource or function. 

### Context

The `Context` abstraction manages the lifecycle of Pulumi operations. It includes the following methods:

- **`create_context`**: Initializes a new context. This should be called at the start of your program.
- **`finish`**: Executes all registered operations. Call this before destroying the context.
- **`destroy_context`**: Cleans up the context. This should be called at the end of your program.
- **`register_resource`**: Registers a new resource in the context.
- **`invoke_resource`**: Invokes a resource (also referred to as a `function`).
- **`add_export`**: Add output as [stack output](https://www.pulumi.com/tutorials/building-with-pulumi/stack-outputs/).

### Output

The `Output` abstraction is used to work with values that are not immediately available (e.g., cloud resource outputs). It provides the following methods:

- **`create_output`**: Creates an `Output` from a known value.
- **`map`**: Applies a function to transform the value inside an `Output`.
- **`combine`**: Combines multiple `Output` objects into a single `Output`.

#### Abstraction Levels for `Output::map`

The `Output::map` function is implemented at different levels of abstraction depending on the language or integration:

##### High Level
- **Description**: Pass a function of type `T1 -> T2` to `Output::map`.
- **Examples**: [Rust language](../languages/rust/index.md).

##### Medium Level
- **Description**: Pass a function of type `String -> String` to `Output::map` where `String` is a serialized JSON value.
- **Examples**: [C FFI](c-ffi.md), [Rust integration](rust.md).

##### Low Level
- **Description**: Pass a function id to `Output::map` and receive it later.
- **Examples**: [Wasm](wasm.md).

### CompositeOutput

`register_resource` and `invoke_resource` return a `CompositeOutput` object. This object is an abstration over map containing resource/function outputs.
For example [Docker image](https://www.pulumi.com/registry/packages/docker/api-docs/image/) will have 7 - `baseImageName`, `context`, `dockerfile`, `id`, `registryServer`, `repoDigest` and `platform`.
Object has the following methods:
 
- **`get_field`**: Get output of map field value
