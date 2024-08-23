use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::worker_route;
use crate::Component;
use std::collections::HashMap;

impl worker_route::Guest for Component {
    fn invoke(name: String, args: worker_route::Args) -> worker_route::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workerRoute:WorkerRoute".into(),
            name,
            object: vec![
                ObjectField {
                    name: "pattern".into(),
                    value: args.pattern,
                },
                ObjectField {
                    name: "scriptName".into(),
                    value: args.script_name,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "pattern".into(),
                },
                ResultField {
                    name: "scriptName".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        worker_route::Res {
            pattern: hashmap.remove("pattern").unwrap(),
            script_name: hashmap.remove("scriptName").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
