use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::r2_bucket;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl r2_bucket::Guest for Component {
    fn invoke(
        name: String,
        args: r2_bucket::Args
    ) -> r2_bucket::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/r2Bucket:R2Bucket".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "location".into(), value: args.location },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "location".into() },
                ResultField { name: "name".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        r2_bucket::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            location: hashmap.remove("location").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
