use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::teams_rule;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl teams_rule::Guest for Component {
    fn invoke(name: String, args: teams_rule::Args) -> teams_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/teamsRule:TeamsRule".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "action".into(), value: args.action },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "devicePosture".into(), value: args.device_posture },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "filters".into(), value: args.filters },
                ObjectField { name: "identity".into(), value: args.identity },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "precedence".into(), value: args.precedence },
                ObjectField { name: "ruleSettings".into(), value: args.rule_settings },
                ObjectField { name: "traffic".into(), value: args.traffic },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "action".into() },
                ResultField { name: "description".into() },
                ResultField { name: "devicePosture".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "filters".into() },
                ResultField { name: "identity".into() },
                ResultField { name: "name".into() },
                ResultField { name: "precedence".into() },
                ResultField { name: "ruleSettings".into() },
                ResultField { name: "traffic".into() },
                ResultField { name: "version".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        teams_rule::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            action: hashmap.remove("action").unwrap(),
            description: hashmap.remove("description").unwrap(),
            device_posture: hashmap.remove("devicePosture").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            filters: hashmap.remove("filters").unwrap(),
            identity: hashmap.remove("identity").unwrap(),
            name: hashmap.remove("name").unwrap(),
            precedence: hashmap.remove("precedence").unwrap(),
            rule_settings: hashmap.remove("ruleSettings").unwrap(),
            traffic: hashmap.remove("traffic").unwrap(),
            version: hashmap.remove("version").unwrap(),
        }

    }
}
