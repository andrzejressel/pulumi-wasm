use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::filter;
use crate::Component;
use std::collections::HashMap;

impl filter::Guest for Component {
    fn invoke(name: String, args: filter::Args) -> filter::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/filter:Filter".into(),
            name,
            object: vec![
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "expression".into(),
                    value: args.expression,
                },
                ObjectField {
                    name: "paused".into(),
                    value: args.paused,
                },
                ObjectField {
                    name: "ref".into(),
                    value: args.ref_,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "expression".into(),
                },
                ResultField {
                    name: "paused".into(),
                },
                ResultField { name: "ref".into() },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        filter::Res {
            description: hashmap.remove("description").unwrap(),
            expression: hashmap.remove("expression").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            ref_: hashmap.remove("ref").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
