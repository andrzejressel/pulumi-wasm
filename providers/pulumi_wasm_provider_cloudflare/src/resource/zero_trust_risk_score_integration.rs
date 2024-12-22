use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_risk_score_integration;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_risk_score_integration::Guest for Component {
    fn invoke(
        name: String,
        args: zero_trust_risk_score_integration::Args
    ) -> zero_trust_risk_score_integration::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustRiskScoreIntegration:ZeroTrustRiskScoreIntegration".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "active".into(), value: args.active },
                ObjectField { name: "integrationType".into(), value: args.integration_type },
                ObjectField { name: "referenceId".into(), value: args.reference_id },
                ObjectField { name: "tenantUrl".into(), value: args.tenant_url },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "active".into() },
                ResultField { name: "integrationType".into() },
                ResultField { name: "referenceId".into() },
                ResultField { name: "tenantUrl".into() },
                ResultField { name: "wellKnownUrl".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        zero_trust_risk_score_integration::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            active: hashmap.remove("active").unwrap(),
            integration_type: hashmap.remove("integrationType").unwrap(),
            reference_id: hashmap.remove("referenceId").unwrap(),
            tenant_url: hashmap.remove("tenantUrl").unwrap(),
            well_known_url: hashmap.remove("wellKnownUrl").unwrap(),
        }
    }
}
