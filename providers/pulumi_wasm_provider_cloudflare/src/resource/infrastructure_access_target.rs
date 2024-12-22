use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::infrastructure_access_target;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl infrastructure_access_target::Guest for Component {
    fn invoke(
        name: String,
        args: infrastructure_access_target::Args
    ) -> infrastructure_access_target::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/infrastructureAccessTarget:InfrastructureAccessTarget".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "ip".into(), value: args.ip },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "createdAt".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "ip".into() },
                ResultField { name: "modifiedAt".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        infrastructure_access_target::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            created_at: hashmap.remove("createdAt").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            ip: hashmap.remove("ip").unwrap(),
            modified_at: hashmap.remove("modifiedAt").unwrap(),
        }
    }
}
