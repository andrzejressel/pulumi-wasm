use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::tiered_cache;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl tiered_cache::Guest for Component {
    fn invoke(name: String, args: tiered_cache::Args) -> tiered_cache::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/tieredCache:TieredCache".into(),
            name,
            object: vec![
                ObjectField { name: "cacheType".into(), value: args.cache_type },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "cacheType".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tiered_cache::Res {
            cache_type: hashmap.remove("cacheType").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
