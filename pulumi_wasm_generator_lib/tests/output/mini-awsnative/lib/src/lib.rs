use pulumi_wasm_rust::Output;
pub mod resource;
pub mod types;

mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "aws-native-pulumi-client",
    });
}

