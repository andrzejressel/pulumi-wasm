use pulumi_wasm_rust::Output;
use crate::bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
pub mod resource;
pub mod types;

mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "docker-pulumi-client",
        with: {
            "component:pulumi-wasm/output-interface@0.1.0": generate
        }
    });
}

fn random_to_domain_mapper<F: serde::Serialize>(random: WitOutput) -> Output<F> {
    unsafe {
        let inner = random.take_handle();
        Output::<F>::new_from_handle(inner)
    }
}

fn clone<T>(output: Output<T>) -> WitOutput {
    unsafe {
        let inner = output.get_inner();
        let cloned = inner.duplicate();
        WitOutput::from_handle(cloned.take_handle())
    }
}