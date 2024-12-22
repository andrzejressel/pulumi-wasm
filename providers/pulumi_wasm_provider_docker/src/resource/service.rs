use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::service;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl service::Guest for Component {
    fn invoke(
        name: String,
        args: service::Args
    ) -> service::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/service:Service".into(),
            name,
            object: vec![
                ObjectField { name: "auth".into(), value: args.auth },
                ObjectField { name: "convergeConfig".into(), value: args.converge_config },
                ObjectField { name: "endpointSpec".into(), value: args.endpoint_spec },
                ObjectField { name: "labels".into(), value: args.labels },
                ObjectField { name: "mode".into(), value: args.mode },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "rollbackConfig".into(), value: args.rollback_config },
                ObjectField { name: "taskSpec".into(), value: args.task_spec },
                ObjectField { name: "updateConfig".into(), value: args.update_config },
            ],
            results: vec![
                ResultField { name: "auth".into() },
                ResultField { name: "convergeConfig".into() },
                ResultField { name: "endpointSpec".into() },
                ResultField { name: "labels".into() },
                ResultField { name: "mode".into() },
                ResultField { name: "name".into() },
                ResultField { name: "rollbackConfig".into() },
                ResultField { name: "taskSpec".into() },
                ResultField { name: "updateConfig".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        service::Res {
            auth: hashmap.remove("auth").unwrap(),
            converge_config: hashmap.remove("convergeConfig").unwrap(),
            endpoint_spec: hashmap.remove("endpointSpec").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            name: hashmap.remove("name").unwrap(),
            rollback_config: hashmap.remove("rollbackConfig").unwrap(),
            task_spec: hashmap.remove("taskSpec").unwrap(),
            update_config: hashmap.remove("updateConfig").unwrap(),
        }

    }
}
