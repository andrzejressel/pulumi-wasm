use log::info;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest};
use crate::bindings::exports::component::pulumi_wasm::pulumi_provider_random_interface::{Guest, RandomStringArgs, RandomStringResult};

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl Guest for Component {
    fn create_random_string(args: RandomStringArgs) -> RandomStringResult {
        wasm_common::setup_logger();
        let r#type = "random:index/randomString:RandomString".to_string();

        info!("Creating random string with name: [{}], args: [{args:?}]", args.name);

        let request = RegisterResourceRequest {
            type_: r#type,
            name: args.name,
            object: vec![ObjectField { name: "length".to_string(), value: args.length }],
        };

        let o = register(&request);
        let result_output = o.get_field("result");

        RandomStringResult {
            result: result_output,
        }
    }

    fn handle_functions() {
        todo!()
    }
}
