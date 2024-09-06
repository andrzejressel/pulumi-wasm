use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::email_routing_catch_all;
use crate::Component;
use std::collections::HashMap;

impl email_routing_catch_all::Guest for Component {
    fn invoke(name: String, args: email_routing_catch_all::Args) -> email_routing_catch_all::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingCatchAll:EmailRoutingCatchAll".into(),
            name,
            object: vec![
                ObjectField {
                    name: "actions".into(),
                    value: args.actions,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "matchers".into(),
                    value: args.matchers,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "actions".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "matchers".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField { name: "tag".into() },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        email_routing_catch_all::Res {
            actions: hashmap.remove("actions").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            matchers: hashmap.remove("matchers").unwrap(),
            name: hashmap.remove("name").unwrap(),
            tag: hashmap.remove("tag").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
