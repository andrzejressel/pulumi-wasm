use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::access_group;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl access_group::Guest for Component {
    fn invoke(
        name: String,
        args: access_group::Args
    ) -> access_group::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessGroup:AccessGroup".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "excludes".into(), value: args.excludes },
                ObjectField { name: "includes".into(), value: args.includes },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "requires".into(), value: args.requires },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "excludes".into() },
                ResultField { name: "includes".into() },
                ResultField { name: "name".into() },
                ResultField { name: "requires".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        access_group::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            excludes: hashmap.remove("excludes").unwrap(),
            includes: hashmap.remove("includes").unwrap(),
            name: hashmap.remove("name").unwrap(),
            requires: hashmap.remove("requires").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
