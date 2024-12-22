use std::collections::HashMap;
use crate::bindings::exports::pulumi::random::random_integer;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl random_integer::Guest for Component {
    fn invoke(
        name: String,
        args: random_integer::Args
    ) -> random_integer::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomInteger:RandomInteger".into(),
            name,
            object: vec![
                ObjectField { name: "keepers".into(), value: args.keepers },
                ObjectField { name: "max".into(), value: args.max },
                ObjectField { name: "min".into(), value: args.min },
                ObjectField { name: "seed".into(), value: args.seed },
            ],
            results: vec![
                ResultField { name: "keepers".into() },
                ResultField { name: "max".into() },
                ResultField { name: "min".into() },
                ResultField { name: "result".into() },
                ResultField { name: "seed".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        random_integer::Res {
            keepers: hashmap.remove("keepers").unwrap(),
            max: hashmap.remove("max").unwrap(),
            min: hashmap.remove("min").unwrap(),
            result: hashmap.remove("result").unwrap(),
            seed: hashmap.remove("seed").unwrap(),
        }
    }
}
