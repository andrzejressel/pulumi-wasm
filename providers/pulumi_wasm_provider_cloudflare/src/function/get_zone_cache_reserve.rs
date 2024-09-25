use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_zone_cache_reserve;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_zone_cache_reserve::Guest for Component {
    fn invoke(
        name: String,
        args: get_zone_cache_reserve::Args
    ) -> get_zone_cache_reserve::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getZoneCacheReserve:getZoneCacheReserve".into(),
            object: vec![
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "enabled".into() },
                ResultField { name: "id".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_zone_cache_reserve::Res {
            enabled: hashmap.remove("enabled").unwrap(),
            id: hashmap.remove("id").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
