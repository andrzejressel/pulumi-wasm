# Rust

Support for Rust is provided in two flavors:

- [Native](native.md)
- [Wasm](wasm.md)

If you don't know which one to choose, go with the native version. Currently, Wasm doesn’t give any benefits and is more complex to use.

# Quick start

!!! note "TL/DR"
    Example project is available at [pulumi-gestalt-example](https://github.com/andrzejressel/pulumi-gestalt-example)


* Add repository

```toml title=".cargo/config.toml"
[registries.pulumi-gestalt]
index = "sparse+https://cargo.cloudsmith.io/andrzej-ressel-github/pulumi-gestalt/"
```

* Add required dependencies

```toml title="Cargo.toml"
[dependencies]
pulumi_gestalt_rust = {version = "=<PULUMI_GESTALT_VERSION>", registry = "pulumi-gestalt"}
anyhow = "1.0.95"
bon = "3.3.1"

[build-dependencies]
pulumi_gestalt_build = { version = "=<PULUMI_GESTALT_VERSION>", registry = "pulumi-gestalt" }
```

* Generate provider code

```rust title="build.rs"
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    pulumi_gestalt_build::generate("random", "4.15.0")?;
    Ok(())
}
```

* Include provider code

```rust title="src/random.rs"
pulumi_gestalt_rust::include_provider!("random");
```

* Use provider

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