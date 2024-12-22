use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::access_tag;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl access_tag::Guest for Component {
    fn invoke(
        name: String,
        args: access_tag::Args
    ) -> access_tag::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessTag:AccessTag".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "appCount".into(), value: args.app_count },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "appCount".into() },
                ResultField { name: "name".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_tag::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            app_count: hashmap.remove("appCount").unwrap(),
            name: hashmap.remove("name").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
