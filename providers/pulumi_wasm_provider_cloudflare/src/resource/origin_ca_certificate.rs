use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::origin_ca_certificate;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl origin_ca_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: origin_ca_certificate::Args
    ) -> origin_ca_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/originCaCertificate:OriginCaCertificate".into(),
            name,
            object: vec![
                ObjectField { name: "csr".into(), value: args.csr },
                ObjectField { name: "hostnames".into(), value: args.hostnames },
                ObjectField { name: "minDaysForRenewal".into(), value: args.min_days_for_renewal },
                ObjectField { name: "requestType".into(), value: args.request_type },
                ObjectField { name: "requestedValidity".into(), value: args.requested_validity },
            ],
            results: vec![
                ResultField { name: "certificate".into() },
                ResultField { name: "csr".into() },
                ResultField { name: "expiresOn".into() },
                ResultField { name: "hostnames".into() },
                ResultField { name: "minDaysForRenewal".into() },
                ResultField { name: "requestType".into() },
                ResultField { name: "requestedValidity".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        origin_ca_certificate::Res {
            certificate: hashmap.remove("certificate").unwrap(),
            csr: hashmap.remove("csr").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            hostnames: hashmap.remove("hostnames").unwrap(),
            min_days_for_renewal: hashmap.remove("minDaysForRenewal").unwrap(),
            request_type: hashmap.remove("requestType").unwrap(),
            requested_validity: hashmap.remove("requestedValidity").unwrap(),
        }
    }
}
