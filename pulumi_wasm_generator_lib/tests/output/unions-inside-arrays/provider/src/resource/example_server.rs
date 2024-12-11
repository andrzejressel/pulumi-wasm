use std::collections::HashMap;
use crate::bindings::exports::pulumi::example::example_server;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl example_server::Guest for Component {
    fn invoke(name: String, args: example_server::Args) -> example_server::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "example:index:ExampleServer".into(),
            name,
            object: vec![
                ObjectField { name: "propertiesCollection".into(), value: args.properties_collection },
            ],
            results: vec![
                ResultField { name: "name".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        example_server::Res {
            name: hashmap.remove("name").unwrap(),
        }

    }
}
