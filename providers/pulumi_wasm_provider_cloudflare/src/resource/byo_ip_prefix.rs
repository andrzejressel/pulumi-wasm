use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::byo_ip_prefix;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl byo_ip_prefix::Guest for Component {
    fn invoke(name: String, args: byo_ip_prefix::Args) -> byo_ip_prefix::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/byoIpPrefix:ByoIpPrefix".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "advertisement".into(), value: args.advertisement },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "prefixId".into(), value: args.prefix_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "advertisement".into() },
                ResultField { name: "description".into() },
                ResultField { name: "prefixId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        byo_ip_prefix::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            advertisement: hashmap.remove("advertisement").unwrap(),
            description: hashmap.remove("description").unwrap(),
            prefix_id: hashmap.remove("prefixId").unwrap(),
        }

    }
}
