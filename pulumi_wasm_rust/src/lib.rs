use crate::output::Output;
pub use pulumi_wasm_rust_macro::pulumi_main;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "pulumi-wasm-rust",
        path: "../wits/world.wit"
    });
}

pub mod output;
pub mod runner;

pub fn add_export<T>(name: &str, output: Output<T>) {
    output.add_to_export(name);
}
