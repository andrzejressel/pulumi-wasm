use pulumi_wasm_rust::Output;
mod types;
pub use types::*;
mod resource;
pub use resource::*;

mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "example-pulumi-client",
    });
}

