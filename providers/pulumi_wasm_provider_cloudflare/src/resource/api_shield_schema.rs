use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::api_shield_schema;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl api_shield_schema::Guest for Component {
    fn invoke(
        name: String,
        args: api_shield_schema::Args
    ) -> api_shield_schema::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldSchema:ApiShieldSchema".into(),
            name,
            object: vec![
                ObjectField { name: "kind".into(), value: args.kind },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "source".into(), value: args.source },
                ObjectField { name: "validationEnabled".into(), value: args.validation_enabled },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "kind".into() },
                ResultField { name: "name".into() },
                ResultField { name: "source".into() },
                ResultField { name: "validationEnabled".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield_schema::Res {
            kind: hashmap.remove("kind").unwrap(),
            name: hashmap.remove("name").unwrap(),
            source: hashmap.remove("source").unwrap(),
            validation_enabled: hashmap.remove("validationEnabled").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
