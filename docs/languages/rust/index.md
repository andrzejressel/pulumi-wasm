# Rust

!!! note "Please Read First"
    Before processing, make sure to learn about [Pulumi](https://www.pulumi.com/tutorials/)

!!! note "TL/DR"
    Example project is available at [pulumi-gestalt-example](https://github.com/andrzejressel/pulumi-gestalt-example)


Support for Rust is provided in two flavors:

- **Native**
- **Wasm**

If you don't know which one to choose, go with the native version. Currently, Wasm doesn’t give any benefits and is more complex to use.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Just](https://github.com/casey/just)
- Pulumi Gestalt language plugin: `pulumi plugin install language gestalt "<PULUMI_GESTALT_VERSION>" --server github://api.github.com/andrzejressel/pulumi-gestalt-releases`
- (Wasm only) Pulumi Gestalt Wasm Runner: `cargo binstall -y --index "sparse+https://cargo.cloudsmith.io/andrzej-ressel-github/pulumi-gestalt/" pulumi_gestalt_wasm_runner@<PULUMI_GESTALT_VERSION>`

## Project setup

### Add Repository

```toml title=".cargo/config.toml"
[registries.pulumi-gestalt]
index = "sparse+https://cargo.cloudsmith.io/andrzej-ressel-github/pulumi-gestalt/"
```

### Add dependencies

```toml title="Cargo.toml"
[dependencies]
pulumi_gestalt_rust = {version = "=<PULUMI_GESTALT_VERSION>", registry = "pulumi-gestalt"}
anyhow = "1.0.95"
bon = "3.3.1"

[build-dependencies]
pulumi_gestalt_build = { version = "=<PULUMI_GESTALT_VERSION>", registry = "pulumi-gestalt" }
```

### Generate provider code

```rust title="build.rs"
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    pulumi_gestalt_build::generate("random", "4.15.0")?;
    Ok(())
}
```

### Include provider code

```rust title="src/random.rs"
pulumi_gestalt_rust::include_provider!("random");
```

### Use provider

=== "Native"

    ```rust title="src/main.rs"
    mod random;
    use anyhow::Result;
    use random::random_string;
    use random::random_string::RandomStringArgs;
    use pulumi_gestalt_rust::*;
    
    pulumi_main!();
    
    fn pulumi_main(context: &Context) -> Result<()> {
        let length: Output<i32> = context.new_output(&4);
        let random_string_1 = random_string::create(
            context,
            "test_1",
            RandomStringArgs::builder().length(length).build_struct(),
        );
    
        let new_length = random_string_1.result.map(|s| s.len() as i32);
    
        let random_string_2 = random_string::create(
            context,
            "test_2",
            RandomStringArgs::builder()
                .length(new_length)
                .build_struct(),
        );
    
        let random_string_3 = random_string::create(
            context,
            "test_3",
            RandomStringArgs::builder()
                .length(random_string_2.length.map(|i| i * 2))
                .build_struct(),
        );
    
        add_export("result", &random_string_1.result);
        add_export("number_1", &random_string_1.length);
        add_export("number_2", &random_string_2.length);
        add_export("number_3", &random_string_3.length);
        Ok(())
    }
    ```

=== "Wasm"
    ```rust title="src/lib.rs"
    mod random;
    use anyhow::Result;
    use random::random_string;
    use random::random_string::RandomStringArgs;
    use pulumi_gestalt_rust::*;
    
    pulumi_main!();
    
    fn pulumi_main(context: &Context) -> Result<()> {
        let length: Output<i32> = context.new_output(&4);
        let random_string_1 = random_string::create(
            context,
            "test_1",
            RandomStringArgs::builder().length(length).build_struct(),
        );
    
        let new_length = random_string_1.result.map(|s| s.len() as i32);
    
        let random_string_2 = random_string::create(
            context,
            "test_2",
            RandomStringArgs::builder()
                .length(new_length)
                .build_struct(),
        );
    
        let random_string_3 = random_string::create(
            context,
            "test_3",
            RandomStringArgs::builder()
                .length(random_string_2.length.map(|i| i * 2))
                .build_struct(),
        );
    
        add_export("result", &random_string_1.result);
        add_export("number_1", &random_string_1.length);
        add_export("number_2", &random_string_2.length);
        add_export("number_3", &random_string_3.length);
        Ok(())
    }
    ```

(the difference is in the `main.rs` vs `lib.rs` file name)

### Add Pulumi.yaml


```yaml title="Pulumi.yaml"
name: Pulumi-Gestalt-Example
runtime: gestalt
```

### Add justfile

=== "Native"

    ```justfile title="justfile" 
    run:
        cargo run
    ```

=== "Wasm"
    ```justfile title="justfile" 
    binary := "pulumi_gestalt_wasm_runner"
    wasm := "target/wasm32-wasip2/debug/<PROJECT_NAME>.wasm"
    WASI_TARGET := "wasm32-wasip2"
    
    run:
        cargo build --target={{WASI_TARGET}}
        {{binary}} run --debug "{{wasm}}"
    ```


You can now setup Pulumi stack using `pulumi stack` and run program using `pulumi up`