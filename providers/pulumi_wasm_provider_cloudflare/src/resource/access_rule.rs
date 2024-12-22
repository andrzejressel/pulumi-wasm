use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::access_rule;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl access_rule::Guest for Component {
    fn invoke(
        name: String,
        args: access_rule::Args
    ) -> access_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessRule:AccessRule".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "configuration".into(), value: args.configuration },
                ObjectField { name: "mode".into(), value: args.mode },
                ObjectField { name: "notes".into(), value: args.notes },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "configuration".into() },
                ResultField { name: "mode".into() },
                ResultField { name: "notes".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        access_rule::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            configuration: hashmap.remove("configuration").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            notes: hashmap.remove("notes").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
