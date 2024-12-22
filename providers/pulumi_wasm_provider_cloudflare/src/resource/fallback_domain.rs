use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::fallback_domain;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl fallback_domain::Guest for Component {
    fn invoke(
        name: String,
        args: fallback_domain::Args
    ) -> fallback_domain::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/fallbackDomain:FallbackDomain".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "domains".into(), value: args.domains },
                ObjectField { name: "policyId".into(), value: args.policy_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "domains".into() },
                ResultField { name: "policyId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        fallback_domain::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            domains: hashmap.remove("domains").unwrap(),
            policy_id: hashmap.remove("policyId").unwrap(),
        }
    }
}
