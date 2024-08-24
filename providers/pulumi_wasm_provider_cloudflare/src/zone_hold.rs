use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::zone_hold;
use crate::Component;
use std::collections::HashMap;

impl zone_hold::Guest for Component {
    fn invoke(name: String, args: zone_hold::Args) -> zone_hold::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneHold:ZoneHold".into(),
            name,
            object: vec![
                ObjectField {
                    name: "hold".into(),
                    value: args.hold,
                },
                ObjectField {
                    name: "holdAfter".into(),
                    value: args.hold_after,
                },
                ObjectField {
                    name: "includeSubdomains".into(),
                    value: args.include_subdomains,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "hold".into(),
                },
                ResultField {
                    name: "holdAfter".into(),
                },
                ResultField {
                    name: "includeSubdomains".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_hold::Res {
            hold: hashmap.remove("hold").unwrap(),
            hold_after: hashmap.remove("holdAfter").unwrap(),
            include_subdomains: hashmap.remove("includeSubdomains").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
