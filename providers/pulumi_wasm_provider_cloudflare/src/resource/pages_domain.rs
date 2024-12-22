use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::pages_domain;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl pages_domain::Guest for Component {
    fn invoke(
        name: String,
        args: pages_domain::Args
    ) -> pages_domain::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/pagesDomain:PagesDomain".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "domain".into(), value: args.domain },
                ObjectField { name: "projectName".into(), value: args.project_name },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "domain".into() },
                ResultField { name: "projectName".into() },
                ResultField { name: "status".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        pages_domain::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            domain: hashmap.remove("domain").unwrap(),
            project_name: hashmap.remove("projectName").unwrap(),
            status: hashmap.remove("status").unwrap(),
        }

    }
}
