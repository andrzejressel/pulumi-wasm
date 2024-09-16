use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::waiting_room_settings;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl waiting_room_settings::Guest for Component {
    fn invoke(name: String, args: waiting_room_settings::Args) -> waiting_room_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomSettings:WaitingRoomSettings".into(),
            name,
            object: vec![
                ObjectField { name: "searchEngineCrawlerBypass".into(), value: args.search_engine_crawler_bypass },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "searchEngineCrawlerBypass".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        waiting_room_settings::Res {
            search_engine_crawler_bypass: hashmap.remove("searchEngineCrawlerBypass").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
