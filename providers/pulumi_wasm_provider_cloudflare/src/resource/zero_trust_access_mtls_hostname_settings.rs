use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_access_mtls_hostname_settings;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_access_mtls_hostname_settings::Guest for Component {
    fn invoke(name: String, args: zero_trust_access_mtls_hostname_settings::Args) -> zero_trust_access_mtls_hostname_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessMtlsHostnameSettings:ZeroTrustAccessMtlsHostnameSettings".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "settings".into(), value: args.settings },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "settings".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zero_trust_access_mtls_hostname_settings::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            settings: hashmap.remove("settings").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
