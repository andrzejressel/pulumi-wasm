use std::collections::HashMap;
use crate::bindings::exports::pulumi::example::module_test;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl module_test::Guest for Component {
    fn invoke(
        name: String,
        args: module_test::Args
    ) {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "example:index:moduleTest".into(),
            name,
            object: vec![
                ObjectField { name: "mod1".into(), value: args.mod1 },
                ObjectField { name: "val".into(), value: args.val },
            ],
            results: vec![
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();


    }
}
