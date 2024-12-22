use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::secret;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl secret::Guest for Component {
    fn invoke(
        name: String,
        args: secret::Args
    ) -> secret::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/secret:Secret".into(),
            name,
            object: vec![
                ObjectField { name: "data".into(), value: args.data },
                ObjectField { name: "labels".into(), value: args.labels },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "data".into() },
                ResultField { name: "labels".into() },
                ResultField { name: "name".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        secret::Res {
            data: hashmap.remove("data").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
