use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::service_config;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl service_config::Guest for Component {
    fn invoke(
        name: String,
        args: service_config::Args
    ) -> service_config::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/serviceConfig:ServiceConfig".into(),
            name,
            object: vec![
                ObjectField { name: "data".into(), value: args.data },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "data".into() },
                ResultField { name: "name".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        service_config::Res {
            data: hashmap.remove("data").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
