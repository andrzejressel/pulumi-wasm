use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::authenticated_origin_pulls;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl authenticated_origin_pulls::Guest for Component {
    fn invoke(
        name: String,
        args: authenticated_origin_pulls::Args
    ) -> authenticated_origin_pulls::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls".into(),
            name,
            object: vec![
                ObjectField { name: "authenticatedOriginPullsCertificate".into(), value: args.authenticated_origin_pulls_certificate },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "authenticatedOriginPullsCertificate".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        authenticated_origin_pulls::Res {
            authenticated_origin_pulls_certificate: hashmap.remove("authenticatedOriginPullsCertificate").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
