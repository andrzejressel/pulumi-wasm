use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::firewall_rule;
use crate::Component;
use std::collections::HashMap;

impl firewall_rule::Guest for Component {
    fn invoke(name: String, args: firewall_rule::Args) -> firewall_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/firewallRule:FirewallRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "action".into(),
                    value: args.action,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "filterId".into(),
                    value: args.filter_id,
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
                    name: "products".into(),
                    value: args.products,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "action".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "filterId".into(),
                },
                ResultField {
                    name: "paused".into(),
                },
                ResultField {
                    name: "priority".into(),
                },
                ResultField {
                    name: "products".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        firewall_rule::Res {
            action: hashmap.remove("action").unwrap(),
            description: hashmap.remove("description").unwrap(),
            filter_id: hashmap.remove("filterId").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            products: hashmap.remove("products").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
