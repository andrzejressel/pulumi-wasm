use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::email_routing_rule;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl email_routing_rule::Guest for Component {
    fn invoke(
        name: String,
        args: email_routing_rule::Args
    ) -> email_routing_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingRule:EmailRoutingRule".into(),
            name,
            object: vec![
                ObjectField { name: "actions".into(), value: args.actions },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "matchers".into(), value: args.matchers },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "priority".into(), value: args.priority },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "actions".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "matchers".into() },
                ResultField { name: "name".into() },
                ResultField { name: "priority".into() },
                ResultField { name: "tag".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        email_routing_rule::Res {
            actions: hashmap.remove("actions").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            matchers: hashmap.remove("matchers").unwrap(),
            name: hashmap.remove("name").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            tag: hashmap.remove("tag").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
