use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_origin_ca_root_certificate;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_origin_ca_root_certificate::Guest for Component {
    fn invoke(
        args: get_origin_ca_root_certificate::Args
    ) -> get_origin_ca_root_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getOriginCaRootCertificate:getOriginCaRootCertificate".into(),
            object: vec![
                ObjectField { name: "algorithm".into(), value: args.algorithm },
            ],
            results: vec![
                ResultField { name: "algorithm".into() },
                ResultField { name: "certPem".into() },
                ResultField { name: "id".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_origin_ca_root_certificate::Res {
            algorithm: hashmap.remove("algorithm").unwrap(),
            cert_pem: hashmap.remove("certPem").unwrap(),
            id: hashmap.remove("id").unwrap(),
        }
    }
}
