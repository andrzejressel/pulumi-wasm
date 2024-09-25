use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::get_network;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_network::Guest for Component {
    fn invoke(
        name: String,
        args: get_network::Args
    ) -> get_network::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "docker:index/getNetwork:getNetwork".into(),
            object: vec![
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "driver".into() },
                ResultField { name: "id".into() },
                ResultField { name: "internal".into() },
                ResultField { name: "ipamConfigs".into() },
                ResultField { name: "name".into() },
                ResultField { name: "options".into() },
                ResultField { name: "scope".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_network::Res {
            driver: hashmap.remove("driver").unwrap(),
            id: hashmap.remove("id").unwrap(),
            internal: hashmap.remove("internal").unwrap(),
            ipam_configs: hashmap.remove("ipamConfigs").unwrap(),
            name: hashmap.remove("name").unwrap(),
            options: hashmap.remove("options").unwrap(),
            scope: hashmap.remove("scope").unwrap(),
        }

    }
}
