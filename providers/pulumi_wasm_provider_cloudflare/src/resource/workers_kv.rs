use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::workers_kv;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl workers_kv::Guest for Component {
    fn invoke(
        name: String,
        args: workers_kv::Args
    ) -> workers_kv::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workersKv:WorkersKv".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "key".into(), value: args.key },
                ObjectField { name: "namespaceId".into(), value: args.namespace_id },
                ObjectField { name: "value".into(), value: args.value },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "key".into() },
                ResultField { name: "namespaceId".into() },
                ResultField { name: "value".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        workers_kv::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            key: hashmap.remove("key").unwrap(),
            namespace_id: hashmap.remove("namespaceId").unwrap(),
            value: hashmap.remove("value").unwrap(),
        }

    }
}
