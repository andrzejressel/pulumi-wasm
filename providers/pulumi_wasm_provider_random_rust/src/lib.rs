use pulumi_wasm_rust::Output;
use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
pub mod resource;
pub mod types;

mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "random-pulumi-client",
        with: {
            "component:pulumi-wasm/output-interface@0.0.0-DEV": pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        }
    });
}

fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
    unsafe { Output::<F>::new_from_handle(output) }
}
