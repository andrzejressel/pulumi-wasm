use pulumi_wasm_rust::Output;
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
mod types;
pub use types::*;

mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "aws-native-pulumi-client",
    });
}

