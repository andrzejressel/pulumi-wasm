use std::collections::HashMap;
use crate::bindings::exports::pulumi::plant::tree_nursery;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl tree_nursery::Guest for Component {
    fn invoke(name: String, args: tree_nursery::Args) {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "plant:tree/v1:Nursery".into(),
            name,
            object: vec![
                ObjectField { name: "sizes".into(), value: args.sizes },
                ObjectField { name: "varieties".into(), value: args.varieties },
            ],
            results: vec![
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();


    }
}
