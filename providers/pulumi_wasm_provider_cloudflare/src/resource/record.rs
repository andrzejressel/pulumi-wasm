use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::record;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl record::Guest for Component {
    fn invoke(name: String, args: record::Args) -> record::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/record:Record".into(),
            name,
            object: vec![
                ObjectField { name: "allowOverwrite".into(), value: args.allow_overwrite },
                ObjectField { name: "comment".into(), value: args.comment },
                ObjectField { name: "content".into(), value: args.content },
                ObjectField { name: "data".into(), value: args.data },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "priority".into(), value: args.priority },
                ObjectField { name: "proxied".into(), value: args.proxied },
                ObjectField { name: "tags".into(), value: args.tags },
                ObjectField { name: "ttl".into(), value: args.ttl },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "value".into(), value: args.value },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "allowOverwrite".into() },
                ResultField { name: "comment".into() },
                ResultField { name: "content".into() },
                ResultField { name: "createdOn".into() },
                ResultField { name: "data".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "metadata".into() },
                ResultField { name: "modifiedOn".into() },
                ResultField { name: "name".into() },
                ResultField { name: "priority".into() },
                ResultField { name: "proxiable".into() },
                ResultField { name: "proxied".into() },
                ResultField { name: "tags".into() },
                ResultField { name: "ttl".into() },
                ResultField { name: "type".into() },
                ResultField { name: "value".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        record::Res {
            allow_overwrite: hashmap.remove("allowOverwrite").unwrap(),
            comment: hashmap.remove("comment").unwrap(),
            content: hashmap.remove("content").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            data: hashmap.remove("data").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            metadata: hashmap.remove("metadata").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            proxiable: hashmap.remove("proxiable").unwrap(),
            proxied: hashmap.remove("proxied").unwrap(),
            tags: hashmap.remove("tags").unwrap(),
            ttl: hashmap.remove("ttl").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            value: hashmap.remove("value").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
