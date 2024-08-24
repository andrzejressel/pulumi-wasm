use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::random::random_bytes;
use crate::Component;
use std::collections::HashMap;

impl random_bytes::Guest for Component {
    fn invoke(name: String, args: random_bytes::Args) -> random_bytes::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomBytes:RandomBytes".into(),
            name,
            object: vec![
                ObjectField {
                    name: "keepers".into(),
                    value: args.keepers,
                },
                ObjectField {
                    name: "length".into(),
                    value: args.length,
                },
            ],
            results: vec![
                ResultField {
                    name: "base64".into(),
                },
                ResultField { name: "hex".into() },
                ResultField {
                    name: "keepers".into(),
                },
                ResultField {
                    name: "length".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_bytes::Res {
            base64: hashmap.remove("base64").unwrap(),
            hex: hashmap.remove("hex").unwrap(),
            keepers: hashmap.remove("keepers").unwrap(),
            length: hashmap.remove("length").unwrap(),
        }
    }
}
