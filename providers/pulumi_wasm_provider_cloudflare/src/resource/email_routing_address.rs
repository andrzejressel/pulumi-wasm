use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::email_routing_address;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl email_routing_address::Guest for Component {
    fn invoke(
        name: String,
        args: email_routing_address::Args
    ) -> email_routing_address::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingAddress:EmailRoutingAddress".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "email".into(), value: args.email },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "created".into() },
                ResultField { name: "email".into() },
                ResultField { name: "modified".into() },
                ResultField { name: "tag".into() },
                ResultField { name: "verified".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        email_routing_address::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            created: hashmap.remove("created").unwrap(),
            email: hashmap.remove("email").unwrap(),
            modified: hashmap.remove("modified").unwrap(),
            tag: hashmap.remove("tag").unwrap(),
            verified: hashmap.remove("verified").unwrap(),
        }
    }
}
