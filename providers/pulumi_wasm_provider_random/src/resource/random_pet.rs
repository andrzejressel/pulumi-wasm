use std::collections::HashMap;
use crate::bindings::exports::pulumi::random::random_pet;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl random_pet::Guest for Component {
    fn invoke(
        name: String,
        args: random_pet::Args
    ) -> random_pet::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomPet:RandomPet".into(),
            name,
            object: vec![
                ObjectField { name: "keepers".into(), value: args.keepers },
                ObjectField { name: "length".into(), value: args.length },
                ObjectField { name: "prefix".into(), value: args.prefix },
                ObjectField { name: "separator".into(), value: args.separator },
            ],
            results: vec![
                ResultField { name: "keepers".into() },
                ResultField { name: "length".into() },
                ResultField { name: "prefix".into() },
                ResultField { name: "separator".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        random_pet::Res {
            keepers: hashmap.remove("keepers").unwrap(),
            length: hashmap.remove("length").unwrap(),
            prefix: hashmap.remove("prefix").unwrap(),
            separator: hashmap.remove("separator").unwrap(),
        }
    }
}
