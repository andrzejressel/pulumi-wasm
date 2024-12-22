use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::worker_secret;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl worker_secret::Guest for Component {
    fn invoke(
        name: String,
        args: worker_secret::Args
    ) -> worker_secret::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workerSecret:WorkerSecret".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "scriptName".into(), value: args.script_name },
                ObjectField { name: "secretText".into(), value: args.secret_text },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "name".into() },
                ResultField { name: "scriptName".into() },
                ResultField { name: "secretText".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        worker_secret::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            name: hashmap.remove("name").unwrap(),
            script_name: hashmap.remove("scriptName").unwrap(),
            secret_text: hashmap.remove("secretText").unwrap(),
        }
    }
}
