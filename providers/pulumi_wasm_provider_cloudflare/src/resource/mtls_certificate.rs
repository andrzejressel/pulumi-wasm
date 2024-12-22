use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::mtls_certificate;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl mtls_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: mtls_certificate::Args
    ) -> mtls_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/mtlsCertificate:MtlsCertificate".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "ca".into(), value: args.ca },
                ObjectField { name: "certificates".into(), value: args.certificates },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "privateKey".into(), value: args.private_key },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "ca".into() },
                ResultField { name: "certificates".into() },
                ResultField { name: "expiresOn".into() },
                ResultField { name: "issuer".into() },
                ResultField { name: "name".into() },
                ResultField { name: "privateKey".into() },
                ResultField { name: "serialNumber".into() },
                ResultField { name: "signature".into() },
                ResultField { name: "uploadedOn".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        mtls_certificate::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            ca: hashmap.remove("ca").unwrap(),
            certificates: hashmap.remove("certificates").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            issuer: hashmap.remove("issuer").unwrap(),
            name: hashmap.remove("name").unwrap(),
            private_key: hashmap.remove("privateKey").unwrap(),
            serial_number: hashmap.remove("serialNumber").unwrap(),
            signature: hashmap.remove("signature").unwrap(),
            uploaded_on: hashmap.remove("uploadedOn").unwrap(),
        }

    }
}
