use std::collections::HashMap;
use crate::bindings::exports::pulumi::random::random_shuffle;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl random_shuffle::Guest for Component {
    fn invoke(
        name: String,
        args: random_shuffle::Args
    ) -> random_shuffle::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomShuffle:RandomShuffle".into(),
            name,
            object: vec![
                ObjectField { name: "inputs".into(), value: args.inputs },
                ObjectField { name: "keepers".into(), value: args.keepers },
                ObjectField { name: "resultCount".into(), value: args.result_count },
                ObjectField { name: "seed".into(), value: args.seed },
            ],
            results: vec![
                ResultField { name: "inputs".into() },
                ResultField { name: "keepers".into() },
                ResultField { name: "resultCount".into() },
                ResultField { name: "results".into() },
                ResultField { name: "seed".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        random_shuffle::Res {
            inputs: hashmap.remove("inputs").unwrap(),
            keepers: hashmap.remove("keepers").unwrap(),
            result_count: hashmap.remove("resultCount").unwrap(),
            results: hashmap.remove("results").unwrap(),
            seed: hashmap.remove("seed").unwrap(),
        }
    }
}
