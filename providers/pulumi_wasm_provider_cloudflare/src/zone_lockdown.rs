use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::zone_lockdown;
use crate::Component;
use std::collections::HashMap;

impl zone_lockdown::Guest for Component {
    fn invoke(name: String, args: zone_lockdown::Args) -> zone_lockdown::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneLockdown:ZoneLockdown".into(),
            name,
            object: vec![
                ObjectField {
                    name: "configurations".into(),
                    value: args.configurations,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "paused".into(),
                    value: args.paused,
                },
                ObjectField {
                    name: "priority".into(),
                    value: args.priority,
                },
                ObjectField {
                    name: "urls".into(),
                    value: args.urls,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "configurations".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "paused".into(),
                },
                ResultField {
                    name: "priority".into(),
                },
                ResultField {
                    name: "urls".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_lockdown::Res {
            configurations: hashmap.remove("configurations").unwrap(),
            description: hashmap.remove("description").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            urls: hashmap.remove("urls").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
