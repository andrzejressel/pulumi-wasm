#[doc(inline)]
pub use pulumi_wasm_rust_macro::pulumi_main;
mod output;
pub use output::Output;
pub use output::ToOutput;

#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;
mod oneof;
pub use oneof::OneOf2;
pub use oneof::OneOf3;
pub use oneof::OneOf4;

/// Add given [Output] to [Stack Output](https://www.pulumi.com/tutorials/building-with-pulumi/stack-outputs/)
pub fn add_export<T>(name: &str, output: &Output<T>) {
    output.add_to_export(name);
}

/// Load specific generated provider
///
/// build.rs:
/// ```rust,no_run
/// use std::error::Error;
/// fn main() -> Result<(), Box<dyn Error>> {
///     pulumi_wasm_build::generate("random", "4.15.0")?;
///     Ok(())
/// }
/// ```
///
/// lib.rs
/// ```rust,ignore
/// include_provider!("random");
/// ```
#[macro_export]
macro_rules! include_provider {
    ($file:expr) => {
        include!(concat!(env!("OUT_DIR"), "/pulumi/", $file, "/main.rs"));
    };
}
