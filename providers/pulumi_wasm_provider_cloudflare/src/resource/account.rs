use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::account;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl account::Guest for Component {
    fn invoke(
        name: String,
        args: account::Args
    ) -> account::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/account:Account".into(),
            name,
            object: vec![
                ObjectField { name: "enforceTwofactor".into(), value: args.enforce_twofactor },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "type".into(), value: args.type_ },
            ],
            results: vec![
                ResultField { name: "enforceTwofactor".into() },
                ResultField { name: "name".into() },
                ResultField { name: "type".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        account::Res {
            enforce_twofactor: hashmap.remove("enforceTwofactor").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
