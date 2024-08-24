use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::web_analytics_rule;
use crate::Component;
use std::collections::HashMap;

impl web_analytics_rule::Guest for Component {
    fn invoke(name: String, args: web_analytics_rule::Args) -> web_analytics_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/webAnalyticsRule:WebAnalyticsRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "host".into(),
                    value: args.host,
                },
                ObjectField {
                    name: "inclusive".into(),
                    value: args.inclusive,
                },
                ObjectField {
                    name: "isPaused".into(),
                    value: args.is_paused,
                },
                ObjectField {
                    name: "paths".into(),
                    value: args.paths,
                },
                ObjectField {
                    name: "rulesetId".into(),
                    value: args.ruleset_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "host".into(),
                },
                ResultField {
                    name: "inclusive".into(),
                },
                ResultField {
                    name: "isPaused".into(),
                },
                ResultField {
                    name: "paths".into(),
                },
                ResultField {
                    name: "rulesetId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        web_analytics_rule::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            host: hashmap.remove("host").unwrap(),
            inclusive: hashmap.remove("inclusive").unwrap(),
            is_paused: hashmap.remove("isPaused").unwrap(),
            paths: hashmap.remove("paths").unwrap(),
            ruleset_id: hashmap.remove("rulesetId").unwrap(),
        }
    }
}
