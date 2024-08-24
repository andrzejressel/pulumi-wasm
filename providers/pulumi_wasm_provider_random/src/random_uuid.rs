use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::random::random_uuid;
use crate::Component;
use std::collections::HashMap;

impl random_uuid::Guest for Component {
    fn invoke(name: String, args: random_uuid::Args) -> random_uuid::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomUuid:RandomUuid".into(),
            name,
            object: vec![ObjectField {
                name: "keepers".into(),
                value: args.keepers,
            }],
            results: vec![
                ResultField {
                    name: "keepers".into(),
                },
                ResultField {
                    name: "result".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_uuid::Res {
            keepers: hashmap.remove("keepers").unwrap(),
            result: hashmap.remove("result").unwrap(),
        }
    }
}
