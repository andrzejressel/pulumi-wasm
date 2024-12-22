use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::workers_kv_namespace;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl workers_kv_namespace::Guest for Component {
    fn invoke(
        name: String,
        args: workers_kv_namespace::Args
    ) -> workers_kv_namespace::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workersKvNamespace:WorkersKvNamespace".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "title".into(), value: args.title },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "title".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        workers_kv_namespace::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            title: hashmap.remove("title").unwrap(),
        }
    }
}
