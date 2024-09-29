use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_rulesets;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_rulesets::Guest for Component {
    fn invoke(
        args: get_rulesets::Args
    ) -> get_rulesets::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getRulesets:getRulesets".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "filter".into(), value: args.filter },
                ObjectField { name: "includeRules".into(), value: args.include_rules },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "filter".into() },
                ResultField { name: "id".into() },
                ResultField { name: "includeRules".into() },
                ResultField { name: "rulesets".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_rulesets::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            filter: hashmap.remove("filter").unwrap(),
            id: hashmap.remove("id").unwrap(),
            include_rules: hashmap.remove("includeRules").unwrap(),
            rulesets: hashmap.remove("rulesets").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
