use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::d1_database;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl d1_database::Guest for Component {
    fn invoke(
        name: String,
        args: d1_database::Args
    ) -> d1_database::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/d1Database:D1Database".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "name".into() },
                ResultField { name: "version".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        d1_database::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            name: hashmap.remove("name").unwrap(),
            version: hashmap.remove("version").unwrap(),
        }
    }
}
