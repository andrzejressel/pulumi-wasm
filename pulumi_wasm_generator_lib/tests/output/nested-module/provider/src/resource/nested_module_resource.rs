use std::collections::HashMap;
use crate::bindings::exports::pulumi::foo::nested_module_resource;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl nested_module_resource::Guest for Component {
    fn invoke(
        name: String,
        args: nested_module_resource::Args
    ) -> nested_module_resource::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "foo:nested/module:Resource".into(),
            name,
            object: vec![
                ObjectField { name: "bar".into(), value: args.bar },
            ],
            results: vec![
                ResultField { name: "bar".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        nested_module_resource::Res {
            bar: hashmap.remove("bar").unwrap(),
        }
    }
}
