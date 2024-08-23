use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::regional_tiered_cache;
use crate::Component;
use std::collections::HashMap;

impl regional_tiered_cache::Guest for Component {
    fn invoke(name: String, args: regional_tiered_cache::Args) -> regional_tiered_cache::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/regionalTieredCache:RegionalTieredCache".into(),
            name,
            object: vec![
                ObjectField {
                    name: "value".into(),
                    value: args.value,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "value".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        regional_tiered_cache::Res {
            value: hashmap.remove("value").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
