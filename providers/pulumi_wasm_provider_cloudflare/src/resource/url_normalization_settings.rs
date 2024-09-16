use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::url_normalization_settings;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl url_normalization_settings::Guest for Component {
    fn invoke(name: String, args: url_normalization_settings::Args) -> url_normalization_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/urlNormalizationSettings:UrlNormalizationSettings".into(),
            name,
            object: vec![
                ObjectField { name: "scope".into(), value: args.scope },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "scope".into() },
                ResultField { name: "type".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        url_normalization_settings::Res {
            scope: hashmap.remove("scope").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
