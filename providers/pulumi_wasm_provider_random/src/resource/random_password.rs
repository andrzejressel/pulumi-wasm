use std::collections::HashMap;
use crate::bindings::exports::pulumi::random::random_password;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl random_password::Guest for Component {
    fn invoke(
        name: String,
        args: random_password::Args
    ) -> random_password::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomPassword:RandomPassword".into(),
            name,
            object: vec![
                ObjectField { name: "keepers".into(), value: args.keepers },
                ObjectField { name: "length".into(), value: args.length },
                ObjectField { name: "lower".into(), value: args.lower },
                ObjectField { name: "minLower".into(), value: args.min_lower },
                ObjectField { name: "minNumeric".into(), value: args.min_numeric },
                ObjectField { name: "minSpecial".into(), value: args.min_special },
                ObjectField { name: "minUpper".into(), value: args.min_upper },
                ObjectField { name: "number".into(), value: args.number },
                ObjectField { name: "numeric".into(), value: args.numeric },
                ObjectField { name: "overrideSpecial".into(), value: args.override_special },
                ObjectField { name: "special".into(), value: args.special },
                ObjectField { name: "upper".into(), value: args.upper },
            ],
            results: vec![
                ResultField { name: "bcryptHash".into() },
                ResultField { name: "keepers".into() },
                ResultField { name: "length".into() },
                ResultField { name: "lower".into() },
                ResultField { name: "minLower".into() },
                ResultField { name: "minNumeric".into() },
                ResultField { name: "minSpecial".into() },
                ResultField { name: "minUpper".into() },
                ResultField { name: "number".into() },
                ResultField { name: "numeric".into() },
                ResultField { name: "overrideSpecial".into() },
                ResultField { name: "result".into() },
                ResultField { name: "special".into() },
                ResultField { name: "upper".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_password::Res {
            bcrypt_hash: hashmap.remove("bcryptHash").unwrap(),
            keepers: hashmap.remove("keepers").unwrap(),
            length: hashmap.remove("length").unwrap(),
            lower: hashmap.remove("lower").unwrap(),
            min_lower: hashmap.remove("minLower").unwrap(),
            min_numeric: hashmap.remove("minNumeric").unwrap(),
            min_special: hashmap.remove("minSpecial").unwrap(),
            min_upper: hashmap.remove("minUpper").unwrap(),
            number: hashmap.remove("number").unwrap(),
            numeric: hashmap.remove("numeric").unwrap(),
            override_special: hashmap.remove("overrideSpecial").unwrap(),
            result: hashmap.remove("result").unwrap(),
            special: hashmap.remove("special").unwrap(),
            upper: hashmap.remove("upper").unwrap(),
        }

    }
}
