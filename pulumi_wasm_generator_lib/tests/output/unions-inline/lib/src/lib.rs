use pulumi_wasm_rust::Output;
use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
mod resource;
pub use resource::*;
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
mod types;
pub use types::*;

mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "example-pulumi-client",
        with: {
            "component:pulumi-wasm/output-interface@0.0.0-DEV": pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        }
    });
}

fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
    unsafe { Output::<F>::new_from_handle(output) }
}
