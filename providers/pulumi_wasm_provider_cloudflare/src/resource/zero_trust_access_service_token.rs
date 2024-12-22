use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_access_service_token;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_access_service_token::Guest for Component {
    fn invoke(
        name: String,
        args: zero_trust_access_service_token::Args
    ) -> zero_trust_access_service_token::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessServiceToken:ZeroTrustAccessServiceToken".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "duration".into(), value: args.duration },
                ObjectField { name: "minDaysForRenewal".into(), value: args.min_days_for_renewal },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "clientId".into() },
                ResultField { name: "clientSecret".into() },
                ResultField { name: "duration".into() },
                ResultField { name: "expiresAt".into() },
                ResultField { name: "minDaysForRenewal".into() },
                ResultField { name: "name".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zero_trust_access_service_token::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            client_id: hashmap.remove("clientId").unwrap(),
            client_secret: hashmap.remove("clientSecret").unwrap(),
            duration: hashmap.remove("duration").unwrap(),
            expires_at: hashmap.remove("expiresAt").unwrap(),
            min_days_for_renewal: hashmap.remove("minDaysForRenewal").unwrap(),
            name: hashmap.remove("name").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
