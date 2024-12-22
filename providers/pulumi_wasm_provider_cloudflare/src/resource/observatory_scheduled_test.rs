use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::observatory_scheduled_test;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl observatory_scheduled_test::Guest for Component {
    fn invoke(
        name: String,
        args: observatory_scheduled_test::Args
    ) -> observatory_scheduled_test::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/observatoryScheduledTest:ObservatoryScheduledTest".into(),
            name,
            object: vec![
                ObjectField { name: "frequency".into(), value: args.frequency },
                ObjectField { name: "region".into(), value: args.region },
                ObjectField { name: "url".into(), value: args.url },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "frequency".into() },
                ResultField { name: "region".into() },
                ResultField { name: "url".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        observatory_scheduled_test::Res {
            frequency: hashmap.remove("frequency").unwrap(),
            region: hashmap.remove("region").unwrap(),
            url: hashmap.remove("url").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
