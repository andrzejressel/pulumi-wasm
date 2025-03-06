# Overview

!!! note "Walk around the code first"
    Before processing, feel free to do [Rust tutorial](../languages/rust/index.md), open code in your IDE, walk through it and check abstractions behind `Output` and generated provider.


## Abstractions

The `Pulumi Gestalt` SDK provides abstractions for working with Pulumi in a structured way. It is built around three core concepts:

1. **Context**: Represents a single execution environment for Pulumi operations. It is not recommended to use a global context, 
   as doing so may limit compatibility with advanced features like the [Pulumi Automation API](https://www.pulumi.com/docs/iac/using-pulumi/automation-api/).
2. **Output**: Represents values that are resolved asynchronously (e.g., cloud resource outputs). Output is usually connected to underlying `value`. Since `Output` can contains
    any type, `value` is JSON representation of it. For example string `hello world` will we `"hello world"`, while number `42` will be `42`. Combining these two values will yield `["hello world", 42]`.
3. **CompositeOutput**: Represents a map of outputs returned by a resource or function. This object is an abstration over map containing resource/function outputs.
   For example [Docker image](https://www.pulumi.com/registry/packages/docker/api-docs/image/) will have 7 - `baseImageName`, `context`, `dockerfile`, `id`, `registryServer`, `repoDigest` and `platform`.

### Context

The `Context` abstraction manages the lifecycle of Pulumi operations. It includes the following methods:

#### `create_context`

!!! note "In WASM this method is `Context` constructor"

!!! abstract "Initializes a new context. This should be called at the start of your program."

    **🛠️ Signature (non-WASM):**
    ```python
    def create_context() -> Context;
    ```
    **🛠️ Signature (WASM):**
    ```python
    def create_context(in_preview: bool) -> Context;
    ```

    **📥 Parameters:**

    | Name         | Type   | Description                         |
    |--------------|--------|-------------------------------------|
    | `in_preview` | `bool` | Value passed to `pulumi-main::main` |

    **📤 Returns:**
    
    | Type      | Description          |
    |-----------|----------------------|
    | `Context` | Instance of context  |


#### `finish`

!!! warning "C FFI and Rust only"
    This function does exist in WIT, but it has [a completely different signature and meaning](wasm.md#callback-emulation).

!!! abstract "Executes all registered operations. Call this before destroying the context."

    **🛠️ Signature:**
    ```python
    def finish(ctx: Context)
    ```

    **📥 Parameters:**

    | Name  | Type      | Description         |
    |-------|-----------|---------------------|
    | `ctx` | `Context` | Instance of context |

#### `destroy_context`

!!! warning "Only in C FFI"
    This function is only available in C FFI. In Rust and WASM, the context is automatically destroyed.

!!! abstract "Cleans up the context. This should be called at the end of your program."

    **🛠️ Signature:**
    ```python
    def destroy_context(ctx: Context)
    ```

    **📥 Parameters:**

    | Name  | Type      | Description         |
    |-------|-----------|---------------------|
    | `ctx` | `Context` | Instance of context |

#### `create_output`

!!! abstract "Creates an `Output` from a known value"

    **🛠️ Signature:**
    ```python
    def create_output(ctx: Context, value: string, secret: bool) -> Output
    ```

    **📥 Parameters:**

    | Name      | Type      | Description         |
    |-----------|-----------|---------------------|
    | `ctx`     | `Context` | Instance of context |
    | `value`   | `string`  | JSON encoded value  |
    | `secret`  | `bool`    | Mark output as secret |

    **📤 Returns:**

    | Type     | Description                                                                                                       |
    |----------|-------------------------------------------------------------------------------------------------------------------|
    | `Output` | An `Output` containing `value`. Does not have to be freed. It will be freed automatically when destroying context |

#### `register_resource`

!!! abstract "Registers a new resource in the context"

    **🛠️ Signature:**
    ```python
    def register_resource(ctx: Context, type: string, name: string, version: string, inputs: List[ObjectField]) -> CompositeOutput
    ```

    **ObjectField**

    | Name      | Type              | Description         |
    |-----------|-------------------|---------------------|
    | name      | string            | Resource name       |
    | value     | Output            | Resource value      |


    **📥 Parameters:**

    | Name      | Type                | Description                                                  |
    |-----------|---------------------|--------------------------------------------------------------|
    | `ctx`     | `Context`           | Instance of context                                          |
    | `type`    | `string`            | Resource type (i.e `random:index/randomString:RandomString`) |
    | `name`    | `string`            | User's resource name (i.e. `my_resource`)                    |
    | `version` | `string`            | Resource provider version                                    |
    | `inputs`  | `List[ObjectField]` | Resource inputs                                              |

    **📤 Returns:**

    | Type             | Description                                                                                                       |
    |------------------|-------------------------------------------------------------------------------------------------------------------|
    | `CompositeOutput` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

#### `invoke_resource`

!!! abstract "Invokes a resource (also referred to as a `function`)"

    **🛠️ Signature:**
    ```python
    def invoke_resource(ctx: Context, token: string, version: string, inputs: List[ObjectField]) -> CompositeOutput
    ```

    **ObjectField**

    | Name      | Type              | Description         |
    |-----------|-------------------|---------------------|
    | name      | string            | Resource name       |
    | value     | Output            | Resource value      |


    **📥 Parameters:**

    | Name      | Type                | Description                                                       |
    |-----------|---------------------|-------------------------------------------------------------------|
    | `ctx`     | `Context`           | Instance of context                                               |
    | `token`   | `string`            | Resource token (i.e [`docker:index/getNetwork:getNetwork`](https://github.com/pulumi/pulumi-docker/blob/v4.6.1/provider/cmd/pulumi-resource-docker/schema.json#L4395)) |
    | `version` | `string`            | Resource provider version                                         |
    | `inputs`  | `List[ObjectField]` | Resource inputs                                                   |


    **📤 Returns:**

    | Type             | Description                                                                                                       |
    |------------------|-------------------------------------------------------------------------------------------------------------------|
    | `CompositeOutput` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

### Output

#### map

!!! abstract "Applies a function to transform the value inside an `Output`"

    **🛠️ Signature:**
    ```python
    def map(output: Output, func: Union[A => B, String => String, String]) -> Output;
    ```

    **📥 Parameters:**

    | Name     | Type                                                        | Description                            |
    |----------|-------------------------------------------------------------|----------------------------------------|
    | `output` | `Output`                                                    | An `Output` object to transform        |
    | `func`   | One of:<br />`A => B`<br />`string => string`<br />`string` | Function to apply to the `Output`      |

    **📤 Returns:**

    | Type     | Description                                                                                                       |
    |----------|-------------------------------------------------------------------------------------------------------------------|
    | `Output` | An `Output` containing transformed value. Does not have to be freed. It will be freed automatically when destroying context |

#### combine

!!! abstract "Combines multiple `Output` objects to create a single composite `Output`"

    **🛠️ Signature:**
    ```python
    def combine(output: Output, outputs: List[Output]) -> Output;
    ```

    **📥 Parameters:**

    | Name      | Type           | Description                         |
    |-----------|----------------|-------------------------------------|
    | `output`  | `Output`       | `this` output                       |
    | `outputs` | `List[Output]` | List of `Output` objects to combine |

    **📤 Returns:**

    | Type     | Description                                                                                                       |
    |----------|-------------------------------------------------------------------------------------------------------------------|
    | `Output` | An `Output` containing combined values. The structure looks like `[output, outputs[0], outputs[1], ...]` Does not have to be freed. It will be freed automatically when destroying context |


#### add_to_export

!!! abstract "Add output as [stack output](https://www.pulumi.com/tutorials/building-with-pulumi/stack-outputs/)."

    **🛠️ Signature:**
    ```python
    def add_to_export(output: Output, name: string);
    ```

    **📥 Parameters:**

    | Name     | Type     | Description                            |
    |----------|----------|----------------------------------------|
    | `output` | `Output` | `Output` object to add as stack output |
    | `name`   | `string` | Name of the stack output               |


### CompositeOutput

#### get_field

!!! abstract "Get resource operation result value"

    **🛠️ Signature:**
    ```python
    def get_field(output: CompositeOutput, field: string) -> Output;
    ```

    **📥 Parameters:**

    | Name     | Type              | Description                            |
    |----------|-------------------|----------------------------------------|
    | `output` | `CompositeOutput` | `CompositeOutput` object to get field  |
    | `field`  | `string`          | Field name                             |

    **📤 Returns:**

    | Type     | Description                                                                                                       |
    |----------|-------------------------------------------------------------------------------------------------------------------|
    | `Output` | An `Output` containing field value. Does not have to be freed. It will be freed automatically when destroying context |

### Abstraction Levels for `Output::map`

The `Output::map` function is implemented at different levels of abstraction depending on the language or integration:

#### High Level
- **Description**: Pass a function of type `T1 -> T2` to `Output::map`.
- **Examples**: [Rust language](../languages/rust/index.md).

#### Medium Level
- **Description**: Pass a function of type `String -> String` to `Output::map` where `String` is a serialized JSON value.
- **Examples**: [C FFI](c-ffi.md), [Rust integration](rust.md).

#### Low Level
- **Description**: Pass a function id to `Output::map` and receive it later.
- **Examples**: [Wasm](wasm.md).
