use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::custom_pages;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl custom_pages::Guest for Component {
    fn invoke(name: String, args: custom_pages::Args) -> custom_pages::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/customPages:CustomPages".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "state".into(), value: args.state },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "url".into(), value: args.url },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "state".into() },
                ResultField { name: "type".into() },
                ResultField { name: "url".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        custom_pages::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            state: hashmap.remove("state").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            url: hashmap.remove("url").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
