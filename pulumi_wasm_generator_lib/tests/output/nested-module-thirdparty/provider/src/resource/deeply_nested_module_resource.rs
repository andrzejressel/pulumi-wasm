use std::collections::HashMap;
use crate::bindings::exports::pulumi::foo_bar::deeply_nested_module_resource;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl deeply_nested_module_resource::Guest for Component {
    fn invoke(name: String, args: deeply_nested_module_resource::Args) -> deeply_nested_module_resource::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "foo-bar:deeply/nested/module:Resource".into(),
            name,
            object: vec![
                ObjectField { name: "baz".into(), value: args.baz },
            ],
            results: vec![
                ResultField { name: "baz".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        deeply_nested_module_resource::Res {
            baz: hashmap.remove("baz").unwrap(),
        }

    }
}
