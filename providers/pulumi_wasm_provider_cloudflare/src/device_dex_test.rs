use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::device_dex_test;
use crate::Component;
use std::collections::HashMap;

impl device_dex_test::Guest for Component {
    fn invoke(name: String, args: device_dex_test::Args) -> device_dex_test::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/deviceDexTest:DeviceDexTest".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "data".into(),
                    value: args.data,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "interval".into(),
                    value: args.interval,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "created".into(),
                },
                ResultField {
                    name: "data".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "interval".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "updated".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        device_dex_test::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            created: hashmap.remove("created").unwrap(),
            data: hashmap.remove("data").unwrap(),
            description: hashmap.remove("description").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            interval: hashmap.remove("interval").unwrap(),
            name: hashmap.remove("name").unwrap(),
            updated: hashmap.remove("updated").unwrap(),
        }
    }
}
