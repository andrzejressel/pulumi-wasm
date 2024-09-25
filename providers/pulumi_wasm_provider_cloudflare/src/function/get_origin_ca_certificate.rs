use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_origin_ca_certificate;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_origin_ca_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: get_origin_ca_certificate::Args
    ) -> get_origin_ca_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getOriginCaCertificate:getOriginCaCertificate".into(),
            object: vec![
                ObjectField { name: "id".into(), value: args.id },
            ],
            results: vec![
                ResultField { name: "certificate".into() },
                ResultField { name: "expiresOn".into() },
                ResultField { name: "hostnames".into() },
                ResultField { name: "id".into() },
                ResultField { name: "requestType".into() },
                ResultField { name: "revokedAt".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_origin_ca_certificate::Res {
            certificate: hashmap.remove("certificate").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            hostnames: hashmap.remove("hostnames").unwrap(),
            id: hashmap.remove("id").unwrap(),
            request_type: hashmap.remove("requestType").unwrap(),
            revoked_at: hashmap.remove("revokedAt").unwrap(),
        }

    }
}
