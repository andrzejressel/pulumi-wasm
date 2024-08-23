use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::device_managed_networks;
use crate::Component;
use std::collections::HashMap;

impl device_managed_networks::Guest for Component {
    fn invoke(name: String, args: device_managed_networks::Args) -> device_managed_networks::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/deviceManagedNetworks:DeviceManagedNetworks".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "config".into(),
                    value: args.config,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
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
                    name: "config".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "type".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        device_managed_networks::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            config: hashmap.remove("config").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
