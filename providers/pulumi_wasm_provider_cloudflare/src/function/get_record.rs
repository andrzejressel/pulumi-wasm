use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_record;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_record::Guest for Component {
    fn invoke(
        args: get_record::Args
    ) -> get_record::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getRecord:getRecord".into(),
            object: vec![
                ObjectField { name: "content".into(), value: args.content },
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "priority".into(), value: args.priority },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "content".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "id".into() },
                ResultField { name: "priority".into() },
                ResultField { name: "proxiable".into() },
                ResultField { name: "proxied".into() },
                ResultField { name: "ttl".into() },
                ResultField { name: "type".into() },
                ResultField { name: "value".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_record::Res {
            content: hashmap.remove("content").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            id: hashmap.remove("id").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            proxiable: hashmap.remove("proxiable").unwrap(),
            proxied: hashmap.remove("proxied").unwrap(),
            ttl: hashmap.remove("ttl").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            value: hashmap.remove("value").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
