mod output;
pub use output::Output;
pub use output::ToOutput;

#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;
mod context;
mod input_or_output;
mod oneof;
pub use input_or_output::InputOrOutput;

pub use context::PulumiContext;

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

/// Generate boilerplate for WASM Min
///
/// ```rust,no_run
/// use pulumi_wasm_rust::*;
/// use anyhow::Result;
///
/// pulumi_main!();
///
/// fn main(context: &PulumiContext) -> Result<()> {
///    Ok(())
/// }
/// ```
#[macro_export]
macro_rules! pulumi_main {
    () => {
        #[export_name = "component:pulumi-wasm-external/pulumi-main@0.0.0-STABLE-DEV#main"]
        unsafe extern "C" fn __exported(arg: i32) {
            let mapped = arg as u8;

            pulumi_wasm_rust::__private::runner::run(mapped, |engine| main(&engine)).unwrap();
        }
    };
}
