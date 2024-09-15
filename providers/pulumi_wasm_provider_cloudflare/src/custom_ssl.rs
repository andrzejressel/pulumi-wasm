use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::custom_ssl;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl custom_ssl::Guest for Component {
    fn invoke(name: String, args: custom_ssl::Args) -> custom_ssl::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/customSsl:CustomSsl".into(),
            name,
            object: vec![
                ObjectField { name: "customSslOptions".into(), value: args.custom_ssl_options },
                ObjectField { name: "customSslPriorities".into(), value: args.custom_ssl_priorities },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "customSslOptions".into() },
                ResultField { name: "customSslPriorities".into() },
                ResultField { name: "expiresOn".into() },
                ResultField { name: "hosts".into() },
                ResultField { name: "issuer".into() },
                ResultField { name: "modifiedOn".into() },
                ResultField { name: "priority".into() },
                ResultField { name: "signature".into() },
                ResultField { name: "status".into() },
                ResultField { name: "uploadedOn".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        custom_ssl::Res {
            custom_ssl_options: hashmap.remove("customSslOptions").unwrap(),
            custom_ssl_priorities: hashmap.remove("customSslPriorities").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            hosts: hashmap.remove("hosts").unwrap(),
            issuer: hashmap.remove("issuer").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            signature: hashmap.remove("signature").unwrap(),
            status: hashmap.remove("status").unwrap(),
            uploaded_on: hashmap.remove("uploadedOn").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
