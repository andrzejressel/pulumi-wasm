use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::device_posture_rule;
use crate::Component;
use std::collections::HashMap;

impl device_posture_rule::Guest for Component {
    fn invoke(name: String, args: device_posture_rule::Args) -> device_posture_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/devicePostureRule:DevicePostureRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "expiration".into(),
                    value: args.expiration,
                },
                ObjectField {
                    name: "inputs".into(),
                    value: args.inputs,
                },
                ObjectField {
                    name: "matches".into(),
                    value: args.matches,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "schedule".into(),
                    value: args.schedule,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "expiration".into(),
                },
                ResultField {
                    name: "inputs".into(),
                },
                ResultField {
                    name: "matches".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "schedule".into(),
                },
                ResultField {
                    name: "type".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        device_posture_rule::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            expiration: hashmap.remove("expiration").unwrap(),
            inputs: hashmap.remove("inputs").unwrap(),
            matches: hashmap.remove("matches").unwrap(),
            name: hashmap.remove("name").unwrap(),
            schedule: hashmap.remove("schedule").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}