pub use pulumi_wasm_rust_macro::pulumi_main;
mod output;
pub use output::Output;
use pulumi_wasm_wit::bindings;

pub mod runner;

pub const SOME_CONST: &str = "component:pulumi-wasm/pulumi-main@0.0.0-DEV#main";

pub fn add_export<T>(name: &str, output: &Output<T>) {
    output.add_to_export(name);
}
