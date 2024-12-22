use pulumi_wasm_rust::Output;
use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod function;
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod types;

mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "myedgeorder-pulumi-client",
        with: {
            "component:pulumi-wasm/output-interface@0.0.0-DEV": pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        }
    });
}

fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
    unsafe { Output::<F>::new_from_handle(output) }
}
pulumi_wasm_provider_common::generate_string_const!(ConstStringPav2, "Pav2");
pulumi_wasm_provider_common::generate_string_const!(ConstStringPurchase, "Purchase");
