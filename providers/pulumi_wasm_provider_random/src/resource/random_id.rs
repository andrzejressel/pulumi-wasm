use std::collections::HashMap;
use crate::bindings::exports::pulumi::random::random_id;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl random_id::Guest for Component {
    fn invoke(
        name: String,
        args: random_id::Args
    ) -> random_id::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomId:RandomId".into(),
            name,
            object: vec![
                ObjectField { name: "byteLength".into(), value: args.byte_length },
                ObjectField { name: "keepers".into(), value: args.keepers },
                ObjectField { name: "prefix".into(), value: args.prefix },
            ],
            results: vec![
                ResultField { name: "b64Std".into() },
                ResultField { name: "b64Url".into() },
                ResultField { name: "byteLength".into() },
                ResultField { name: "dec".into() },
                ResultField { name: "hex".into() },
                ResultField { name: "keepers".into() },
                ResultField { name: "prefix".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_id::Res {
            b64_std: hashmap.remove("b64Std").unwrap(),
            b64_url: hashmap.remove("b64Url").unwrap(),
            byte_length: hashmap.remove("byteLength").unwrap(),
            dec: hashmap.remove("dec").unwrap(),
            hex: hashmap.remove("hex").unwrap(),
            keepers: hashmap.remove("keepers").unwrap(),
            prefix: hashmap.remove("prefix").unwrap(),
        }

    }
}
