use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::network;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl network::Guest for Component {
    fn invoke(name: String, args: network::Args) -> network::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/network:Network".into(),
            name,
            object: vec![
                ObjectField { name: "attachable".into(), value: args.attachable },
                ObjectField { name: "checkDuplicate".into(), value: args.check_duplicate },
                ObjectField { name: "driver".into(), value: args.driver },
                ObjectField { name: "ingress".into(), value: args.ingress },
                ObjectField { name: "internal".into(), value: args.internal },
                ObjectField { name: "ipamConfigs".into(), value: args.ipam_configs },
                ObjectField { name: "ipamDriver".into(), value: args.ipam_driver },
                ObjectField { name: "ipamOptions".into(), value: args.ipam_options },
                ObjectField { name: "ipv6".into(), value: args.ipv6 },
                ObjectField { name: "labels".into(), value: args.labels },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "options".into(), value: args.options },
            ],
            results: vec![
                ResultField { name: "attachable".into() },
                ResultField { name: "checkDuplicate".into() },
                ResultField { name: "driver".into() },
                ResultField { name: "ingress".into() },
                ResultField { name: "internal".into() },
                ResultField { name: "ipamConfigs".into() },
                ResultField { name: "ipamDriver".into() },
                ResultField { name: "ipamOptions".into() },
                ResultField { name: "ipv6".into() },
                ResultField { name: "labels".into() },
                ResultField { name: "name".into() },
                ResultField { name: "options".into() },
                ResultField { name: "scope".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        network::Res {
            attachable: hashmap.remove("attachable").unwrap(),
            check_duplicate: hashmap.remove("checkDuplicate").unwrap(),
            driver: hashmap.remove("driver").unwrap(),
            ingress: hashmap.remove("ingress").unwrap(),
            internal: hashmap.remove("internal").unwrap(),
            ipam_configs: hashmap.remove("ipamConfigs").unwrap(),
            ipam_driver: hashmap.remove("ipamDriver").unwrap(),
            ipam_options: hashmap.remove("ipamOptions").unwrap(),
            ipv6: hashmap.remove("ipv6").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            name: hashmap.remove("name").unwrap(),
            options: hashmap.remove("options").unwrap(),
            scope: hashmap.remove("scope").unwrap(),
        }

    }
}
