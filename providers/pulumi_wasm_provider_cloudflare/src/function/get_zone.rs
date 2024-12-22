use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_zone;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_zone::Guest for Component {
    fn invoke(
        args: get_zone::Args
    ) -> get_zone::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getZone:getZone".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "id".into() },
                ResultField { name: "name".into() },
                ResultField { name: "nameServers".into() },
                ResultField { name: "paused".into() },
                ResultField { name: "plan".into() },
                ResultField { name: "status".into() },
                ResultField { name: "vanityNameServers".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_zone::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            id: hashmap.remove("id").unwrap(),
            name: hashmap.remove("name").unwrap(),
            name_servers: hashmap.remove("nameServers").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            plan: hashmap.remove("plan").unwrap(),
            status: hashmap.remove("status").unwrap(),
            vanity_name_servers: hashmap.remove("vanityNameServers").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
