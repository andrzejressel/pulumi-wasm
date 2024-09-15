use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zone;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zone::Guest for Component {
    fn invoke(name: String, args: zone::Args) -> zone::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zone:Zone".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "jumpStart".into(), value: args.jump_start },
                ObjectField { name: "paused".into(), value: args.paused },
                ObjectField { name: "plan".into(), value: args.plan },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "zone".into(), value: args.zone },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "jumpStart".into() },
                ResultField { name: "meta".into() },
                ResultField { name: "nameServers".into() },
                ResultField { name: "paused".into() },
                ResultField { name: "plan".into() },
                ResultField { name: "status".into() },
                ResultField { name: "type".into() },
                ResultField { name: "vanityNameServers".into() },
                ResultField { name: "verificationKey".into() },
                ResultField { name: "zone".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            jump_start: hashmap.remove("jumpStart").unwrap(),
            meta: hashmap.remove("meta").unwrap(),
            name_servers: hashmap.remove("nameServers").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            plan: hashmap.remove("plan").unwrap(),
            status: hashmap.remove("status").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            vanity_name_servers: hashmap.remove("vanityNameServers").unwrap(),
            verification_key: hashmap.remove("verificationKey").unwrap(),
            zone: hashmap.remove("zone").unwrap(),
        }

    }
}
