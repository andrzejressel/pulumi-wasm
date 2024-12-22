use std::collections::HashMap;
use crate::bindings::exports::pulumi::plant::tree_v1_nursery;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl tree_v1_nursery::Guest for Component {
    fn invoke(
        name: String,
        args: tree_v1_nursery::Args
    ) {
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

        register(&request);
    }
}
