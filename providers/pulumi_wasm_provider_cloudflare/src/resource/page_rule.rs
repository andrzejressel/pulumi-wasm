use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::page_rule;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl page_rule::Guest for Component {
    fn invoke(
        name: String,
        args: page_rule::Args
    ) -> page_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/pageRule:PageRule".into(),
            name,
            object: vec![
                ObjectField { name: "actions".into(), value: args.actions },
                ObjectField { name: "priority".into(), value: args.priority },
                ObjectField { name: "status".into(), value: args.status },
                ObjectField { name: "target".into(), value: args.target },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "actions".into() },
                ResultField { name: "priority".into() },
                ResultField { name: "status".into() },
                ResultField { name: "target".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        page_rule::Res {
            actions: hashmap.remove("actions").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            status: hashmap.remove("status").unwrap(),
            target: hashmap.remove("target").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
