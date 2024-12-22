use pulumi_wasm_rust::Output;
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod types;

mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "example-pulumi-client",
    });
}

