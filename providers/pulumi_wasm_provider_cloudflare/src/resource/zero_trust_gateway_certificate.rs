use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_gateway_certificate;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_gateway_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: zero_trust_gateway_certificate::Args
    ) -> zero_trust_gateway_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustGatewayCertificate:ZeroTrustGatewayCertificate".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "activate".into(), value: args.activate },
                ObjectField { name: "custom".into(), value: args.custom },
                ObjectField { name: "gatewayManaged".into(), value: args.gateway_managed },
                ObjectField { name: "validityPeriodDays".into(), value: args.validity_period_days },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "activate".into() },
                ResultField { name: "bindingStatus".into() },
                ResultField { name: "createdAt".into() },
                ResultField { name: "custom".into() },
                ResultField { name: "expiresOn".into() },
                ResultField { name: "gatewayManaged".into() },
                ResultField { name: "inUse".into() },
                ResultField { name: "qsPackId".into() },
                ResultField { name: "uploadedOn".into() },
                ResultField { name: "validityPeriodDays".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        zero_trust_gateway_certificate::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            activate: hashmap.remove("activate").unwrap(),
            binding_status: hashmap.remove("bindingStatus").unwrap(),
            created_at: hashmap.remove("createdAt").unwrap(),
            custom: hashmap.remove("custom").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            gateway_managed: hashmap.remove("gatewayManaged").unwrap(),
            in_use: hashmap.remove("inUse").unwrap(),
            qs_pack_id: hashmap.remove("qsPackId").unwrap(),
            uploaded_on: hashmap.remove("uploadedOn").unwrap(),
            validity_period_days: hashmap.remove("validityPeriodDays").unwrap(),
        }
    }
}
