use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::ruleset;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl ruleset::Guest for Component {
    fn invoke(name: String, args: ruleset::Args) -> ruleset::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/ruleset:Ruleset".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "kind".into(), value: args.kind },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "phase".into(), value: args.phase },
                ObjectField { name: "rules".into(), value: args.rules },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "description".into() },
                ResultField { name: "kind".into() },
                ResultField { name: "name".into() },
                ResultField { name: "phase".into() },
                ResultField { name: "rules".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        ruleset::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            kind: hashmap.remove("kind").unwrap(),
            name: hashmap.remove("name").unwrap(),
            phase: hashmap.remove("phase").unwrap(),
            rules: hashmap.remove("rules").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
