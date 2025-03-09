mod output;
pub use output::ToOutput;

#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;
mod input_or_output;
mod oneof;

pub use input_or_output::InputOrOutput;

pub use oneof::OneOf2;
pub use oneof::OneOf3;
pub use oneof::OneOf4;

pub use pulumi_gestalt_rust_adapter::GestaltCompositeOutput;
pub use pulumi_gestalt_rust_adapter::GestaltContext;
pub use pulumi_gestalt_rust_adapter::GestaltOutput;
pub use pulumi_gestalt_rust_adapter::InvokeResourceRequest;
pub use pulumi_gestalt_rust_adapter::RegisterResourceRequest;
pub use pulumi_gestalt_rust_adapter::ResourceRequestObjectField;

#[cfg(target_arch = "wasm32")]
pub type Context = pulumi_gestalt_rust_adapter_wasm::WasmContext;
#[cfg(target_arch = "wasm32")]
pub type Output<T> = pulumi_gestalt_rust_adapter_wasm::WasmOutput<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type Context = pulumi_gestalt_rust_adapter_native::NativeContext;
#[cfg(not(target_arch = "wasm32"))]
pub type Output<T> = pulumi_gestalt_rust_adapter_native::NativeOutput<T>;

/// Add the given [Output] to [Stack Output](https://www.pulumi.com/tutorials/building-with-pulumi/stack-outputs/)
pub fn add_export<T>(name: &str, output: &Output<T>) {
    output.add_to_export(name);
}

/// Load specific generated provider
///
/// build.rs:
/// ```rust,no_run
/// use std::error::Error;
/// fn main() -> Result<(), Box<dyn Error>> {
///     pulumi_gestalt_build::generate("random", "4.15.0")?;
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

/// Generate boilerplate for Wasm entrypoint
///
/// ```rust,no_run
/// use pulumi_gestalt_rust::*;
/// use anyhow::Result;
///
/// pulumi_main!();
///
/// fn pulumi_main(context: &Context) -> Result<()> {
///    Ok(())
/// }
/// ```
#[macro_export]
macro_rules! pulumi_main {
    () => {
        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "component:pulumi-gestalt-external/pulumi-main@0.0.0-STABLE-DEV#main")]
        unsafe extern "C" fn __exported(arg: i32) {
            let mapped = arg as u8;

            pulumi_gestalt_rust::__private::pulumi_gestalt_rust_adapter_wasm::runner::run(
                mapped,
                |engine| pulumi_main(&engine),
            )
            .unwrap();
        }

        #[cfg(not(target_arch = "wasm32"))]
        fn main() {
            let context = Context::new();
            pulumi_main(&context).unwrap();
            context.finish();
        }
    };
}
