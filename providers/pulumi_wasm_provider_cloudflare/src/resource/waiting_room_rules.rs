use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::waiting_room_rules;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl waiting_room_rules::Guest for Component {
    fn invoke(
        name: String,
        args: waiting_room_rules::Args
    ) -> waiting_room_rules::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomRules:WaitingRoomRules".into(),
            name,
            object: vec![
                ObjectField { name: "rules".into(), value: args.rules },
                ObjectField { name: "waitingRoomId".into(), value: args.waiting_room_id },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "rules".into() },
                ResultField { name: "waitingRoomId".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        waiting_room_rules::Res {
            rules: hashmap.remove("rules").unwrap(),
            waiting_room_id: hashmap.remove("waitingRoomId").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
