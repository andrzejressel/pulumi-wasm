use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::cloud_connector_rules;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl cloud_connector_rules::Guest for Component {
    fn invoke(
        name: String,
        args: cloud_connector_rules::Args
    ) -> cloud_connector_rules::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/cloudConnectorRules:CloudConnectorRules".into(),
            name,
            object: vec![
                ObjectField { name: "rules".into(), value: args.rules },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "rules".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        cloud_connector_rules::Res {
            rules: hashmap.remove("rules").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
