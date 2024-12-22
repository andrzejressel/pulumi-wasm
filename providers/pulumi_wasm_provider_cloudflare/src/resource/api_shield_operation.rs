use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::api_shield_operation;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl api_shield_operation::Guest for Component {
    fn invoke(
        name: String,
        args: api_shield_operation::Args
    ) -> api_shield_operation::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldOperation:ApiShieldOperation".into(),
            name,
            object: vec![
                ObjectField { name: "endpoint".into(), value: args.endpoint },
                ObjectField { name: "host".into(), value: args.host },
                ObjectField { name: "method".into(), value: args.method },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "endpoint".into() },
                ResultField { name: "host".into() },
                ResultField { name: "method".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield_operation::Res {
            endpoint: hashmap.remove("endpoint").unwrap(),
            host: hashmap.remove("host").unwrap(),
            method: hashmap.remove("method").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
