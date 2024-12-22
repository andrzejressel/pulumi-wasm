use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::custom_hostname;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl custom_hostname::Guest for Component {
    fn invoke(
        name: String,
        args: custom_hostname::Args
    ) -> custom_hostname::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/customHostname:CustomHostname".into(),
            name,
            object: vec![
                ObjectField { name: "customMetadata".into(), value: args.custom_metadata },
                ObjectField { name: "customOriginServer".into(), value: args.custom_origin_server },
                ObjectField { name: "customOriginSni".into(), value: args.custom_origin_sni },
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "ssls".into(), value: args.ssls },
                ObjectField { name: "waitForSslPendingValidation".into(), value: args.wait_for_ssl_pending_validation },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "customMetadata".into() },
                ResultField { name: "customOriginServer".into() },
                ResultField { name: "customOriginSni".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "ownershipVerification".into() },
                ResultField { name: "ownershipVerificationHttp".into() },
                ResultField { name: "ssls".into() },
                ResultField { name: "status".into() },
                ResultField { name: "waitForSslPendingValidation".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        custom_hostname::Res {
            custom_metadata: hashmap.remove("customMetadata").unwrap(),
            custom_origin_server: hashmap.remove("customOriginServer").unwrap(),
            custom_origin_sni: hashmap.remove("customOriginSni").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            ownership_verification: hashmap.remove("ownershipVerification").unwrap(),
            ownership_verification_http: hashmap.remove("ownershipVerificationHttp").unwrap(),
            ssls: hashmap.remove("ssls").unwrap(),
            status: hashmap.remove("status").unwrap(),
            wait_for_ssl_pending_validation: hashmap.remove("waitForSslPendingValidation").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
