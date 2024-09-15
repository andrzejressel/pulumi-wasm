use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::list;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl list::Guest for Component {
    fn invoke(name: String, args: list::Args) -> list::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/list:List".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "items".into(), value: args.items },
                ObjectField { name: "kind".into(), value: args.kind },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "description".into() },
                ResultField { name: "items".into() },
                ResultField { name: "kind".into() },
                ResultField { name: "name".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        list::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            items: hashmap.remove("items").unwrap(),
            kind: hashmap.remove("kind").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }

    }
}
