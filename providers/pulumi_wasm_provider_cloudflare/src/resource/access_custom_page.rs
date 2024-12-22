use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::access_custom_page;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl access_custom_page::Guest for Component {
    fn invoke(
        name: String,
        args: access_custom_page::Args
    ) -> access_custom_page::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessCustomPage:AccessCustomPage".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "appCount".into(), value: args.app_count },
                ObjectField { name: "customHtml".into(), value: args.custom_html },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "appCount".into() },
                ResultField { name: "customHtml".into() },
                ResultField { name: "name".into() },
                ResultField { name: "type".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        access_custom_page::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            app_count: hashmap.remove("appCount").unwrap(),
            custom_html: hashmap.remove("customHtml").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
