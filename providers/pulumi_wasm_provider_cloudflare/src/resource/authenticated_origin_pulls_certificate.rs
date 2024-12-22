use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::authenticated_origin_pulls_certificate;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl authenticated_origin_pulls_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: authenticated_origin_pulls_certificate::Args
    ) -> authenticated_origin_pulls_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/authenticatedOriginPullsCertificate:AuthenticatedOriginPullsCertificate".into(),
            name,
            object: vec![
                ObjectField { name: "certificate".into(), value: args.certificate },
                ObjectField { name: "privateKey".into(), value: args.private_key },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "certificate".into() },
                ResultField { name: "expiresOn".into() },
                ResultField { name: "issuer".into() },
                ResultField { name: "privateKey".into() },
                ResultField { name: "serialNumber".into() },
                ResultField { name: "signature".into() },
                ResultField { name: "status".into() },
                ResultField { name: "type".into() },
                ResultField { name: "uploadedOn".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        authenticated_origin_pulls_certificate::Res {
            certificate: hashmap.remove("certificate").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            issuer: hashmap.remove("issuer").unwrap(),
            private_key: hashmap.remove("privateKey").unwrap(),
            serial_number: hashmap.remove("serialNumber").unwrap(),
            signature: hashmap.remove("signature").unwrap(),
            status: hashmap.remove("status").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            uploaded_on: hashmap.remove("uploadedOn").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
