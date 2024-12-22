use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::worker_domain;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl worker_domain::Guest for Component {
    fn invoke(
        name: String,
        args: worker_domain::Args
    ) -> worker_domain::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workerDomain:WorkerDomain".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "environment".into(), value: args.environment },
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "service".into(), value: args.service },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "environment".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "service".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        worker_domain::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            environment: hashmap.remove("environment").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            service: hashmap.remove("service").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
