use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::email_routing_settings;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl email_routing_settings::Guest for Component {
    fn invoke(
        name: String,
        args: email_routing_settings::Args
    ) -> email_routing_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingSettings:EmailRoutingSettings".into(),
            name,
            object: vec![
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "skipWizard".into(), value: args.skip_wizard },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "created".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "modified".into() },
                ResultField { name: "name".into() },
                ResultField { name: "skipWizard".into() },
                ResultField { name: "status".into() },
                ResultField { name: "tag".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        email_routing_settings::Res {
            created: hashmap.remove("created").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            modified: hashmap.remove("modified").unwrap(),
            name: hashmap.remove("name").unwrap(),
            skip_wizard: hashmap.remove("skipWizard").unwrap(),
            status: hashmap.remove("status").unwrap(),
            tag: hashmap.remove("tag").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
