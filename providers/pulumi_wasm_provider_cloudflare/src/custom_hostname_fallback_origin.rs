use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::custom_hostname_fallback_origin;
use crate::Component;
use std::collections::HashMap;

impl custom_hostname_fallback_origin::Guest for Component {
    fn invoke(
        name: String,
        args: custom_hostname_fallback_origin::Args,
    ) -> custom_hostname_fallback_origin::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/customHostnameFallbackOrigin:CustomHostnameFallbackOrigin"
                .into(),
            name,
            object: vec![
                ObjectField {
                    name: "origin".into(),
                    value: args.origin,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "origin".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        custom_hostname_fallback_origin::Res {
            origin: hashmap.remove("origin").unwrap(),
            status: hashmap.remove("status").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
