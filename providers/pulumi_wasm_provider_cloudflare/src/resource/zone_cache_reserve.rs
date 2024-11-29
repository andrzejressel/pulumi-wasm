use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zone_cache_reserve;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zone_cache_reserve::Guest for Component {
    fn invoke(name: String, args: zone_cache_reserve::Args) -> zone_cache_reserve::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneCacheReserve:ZoneCacheReserve".into(),
            name,
            object: vec![
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "enabled".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_cache_reserve::Res {
            enabled: hashmap.remove("enabled").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
