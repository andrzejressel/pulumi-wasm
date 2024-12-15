use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_zero_trust_access_identity_provider;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_zero_trust_access_identity_provider::Guest for Component {
    fn invoke(
        args: get_zero_trust_access_identity_provider::Args
    ) -> get_zero_trust_access_identity_provider::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getZeroTrustAccessIdentityProvider:getZeroTrustAccessIdentityProvider".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "id".into() },
                ResultField { name: "name".into() },
                ResultField { name: "type".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_zero_trust_access_identity_provider::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            id: hashmap.remove("id").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
