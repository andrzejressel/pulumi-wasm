use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::hyperdrive_config;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl hyperdrive_config::Guest for Component {
    fn invoke(name: String, args: hyperdrive_config::Args) -> hyperdrive_config::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/hyperdriveConfig:HyperdriveConfig".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "caching".into(), value: args.caching },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "origin".into(), value: args.origin },
                ObjectField { name: "resourceId".into(), value: args.resource_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "caching".into() },
                ResultField { name: "name".into() },
                ResultField { name: "origin".into() },
                ResultField { name: "resourceId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        hyperdrive_config::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            caching: hashmap.remove("caching").unwrap(),
            name: hashmap.remove("name").unwrap(),
            origin: hashmap.remove("origin").unwrap(),
            resource_id: hashmap.remove("resourceId").unwrap(),
        }

    }
}
