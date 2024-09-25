use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_zones;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_zones::Guest for Component {
    fn invoke(
        name: String,
        args: get_zones::Args
    ) -> get_zones::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getZones:getZones".into(),
            object: vec![
                ObjectField { name: "filter".into(), value: args.filter },
            ],
            results: vec![
                ResultField { name: "filter".into() },
                ResultField { name: "id".into() },
                ResultField { name: "zones".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_zones::Res {
            filter: hashmap.remove("filter").unwrap(),
            id: hashmap.remove("id").unwrap(),
            zones: hashmap.remove("zones").unwrap(),
        }

    }
}
