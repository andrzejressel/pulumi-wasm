use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::user_agent_blocking_rule;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl user_agent_blocking_rule::Guest for Component {
    fn invoke(
        name: String,
        args: user_agent_blocking_rule::Args
    ) -> user_agent_blocking_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/userAgentBlockingRule:UserAgentBlockingRule".into(),
            name,
            object: vec![
                ObjectField { name: "configuration".into(), value: args.configuration },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "mode".into(), value: args.mode },
                ObjectField { name: "paused".into(), value: args.paused },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "configuration".into() },
                ResultField { name: "description".into() },
                ResultField { name: "mode".into() },
                ResultField { name: "paused".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        user_agent_blocking_rule::Res {
            configuration: hashmap.remove("configuration").unwrap(),
            description: hashmap.remove("description").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
