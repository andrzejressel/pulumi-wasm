use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_access_short_lived_certificate;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_access_short_lived_certificate::Guest for Component {
    fn invoke(name: String, args: zero_trust_access_short_lived_certificate::Args) -> zero_trust_access_short_lived_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessShortLivedCertificate:ZeroTrustAccessShortLivedCertificate".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "applicationId".into(), value: args.application_id },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "applicationId".into() },
                ResultField { name: "aud".into() },
                ResultField { name: "publicKey".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zero_trust_access_short_lived_certificate::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            application_id: hashmap.remove("applicationId").unwrap(),
            aud: hashmap.remove("aud").unwrap(),
            public_key: hashmap.remove("publicKey").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
