use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_key_access_key_configuration;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_key_access_key_configuration::Guest for Component {
    fn invoke(
        name: String,
        args: zero_trust_key_access_key_configuration::Args
    ) -> zero_trust_key_access_key_configuration::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustKeyAccessKeyConfiguration:ZeroTrustKeyAccessKeyConfiguration".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "keyRotationIntervalDays".into(), value: args.key_rotation_interval_days },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "keyRotationIntervalDays".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        zero_trust_key_access_key_configuration::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            key_rotation_interval_days: hashmap.remove("keyRotationIntervalDays").unwrap(),
        }
    }
}
