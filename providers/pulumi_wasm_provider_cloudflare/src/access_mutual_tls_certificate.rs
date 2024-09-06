use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::access_mutual_tls_certificate;
use crate::Component;
use std::collections::HashMap;

impl access_mutual_tls_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: access_mutual_tls_certificate::Args,
    ) -> access_mutual_tls_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "associatedHostnames".into(),
                    value: args.associated_hostnames,
                },
                ObjectField {
                    name: "certificate".into(),
                    value: args.certificate,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "associatedHostnames".into(),
                },
                ResultField {
                    name: "certificate".into(),
                },
                ResultField {
                    name: "fingerprint".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_mutual_tls_certificate::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            associated_hostnames: hashmap.remove("associatedHostnames").unwrap(),
            certificate: hashmap.remove("certificate").unwrap(),
            fingerprint: hashmap.remove("fingerprint").unwrap(),
            name: hashmap.remove("name").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
