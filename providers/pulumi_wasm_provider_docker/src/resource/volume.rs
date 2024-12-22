use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::volume;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl volume::Guest for Component {
    fn invoke(
        name: String,
        args: volume::Args
    ) -> volume::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/volume:Volume".into(),
            name,
            object: vec![
                ObjectField { name: "driver".into(), value: args.driver },
                ObjectField { name: "driverOpts".into(), value: args.driver_opts },
                ObjectField { name: "labels".into(), value: args.labels },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "driver".into() },
                ResultField { name: "driverOpts".into() },
                ResultField { name: "labels".into() },
                ResultField { name: "mountpoint".into() },
                ResultField { name: "name".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        volume::Res {
            driver: hashmap.remove("driver").unwrap(),
            driver_opts: hashmap.remove("driverOpts").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            mountpoint: hashmap.remove("mountpoint").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
