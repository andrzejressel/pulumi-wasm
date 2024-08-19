use bindings::exports::pulumi::random::random_bytes;
use bindings::exports::pulumi::random::random_id;
use bindings::exports::pulumi::random::random_integer;
use bindings::exports::pulumi::random::random_password;
use bindings::exports::pulumi::random::random_pet;
use bindings::exports::pulumi::random::random_shuffle;
use bindings::exports::pulumi::random::random_string;
use bindings::exports::pulumi::random::random_uuid;
use std::collections::HashMap;

use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl random_bytes::Guest for Component {
    fn invoke(name: String, args: random_bytes::Args) -> random_bytes::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomBytes:RandomBytes".into(),
            name,
            object: vec![
                ObjectField {
                    name: "keepers".into(),
                    value: args.keepers,
                },
                ObjectField {
                    name: "length".into(),
                    value: args.length,
                },
            ],
            results: vec![
                ResultField {
                    name: "base64".into(),
                },
                ResultField { name: "hex".into() },
                ResultField {
                    name: "keepers".into(),
                },
                ResultField {
                    name: "length".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_bytes::Res {
            base64: hashmap.remove("base64").unwrap(),
            hex: hashmap.remove("hex").unwrap(),
            keepers: hashmap.remove("keepers").unwrap(),
            length: hashmap.remove("length").unwrap(),
        }
    }
}
impl random_id::Guest for Component {
    fn invoke(name: String, args: random_id::Args) -> random_id::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomId:RandomId".into(),
            name,
            object: vec![
                ObjectField {
                    name: "byteLength".into(),
                    value: args.byte_length,
                },
                ObjectField {
                    name: "keepers".into(),
                    value: args.keepers,
                },
                ObjectField {
                    name: "prefix".into(),
                    value: args.prefix,
                },
            ],
            results: vec![
                ResultField {
                    name: "b64Std".into(),
                },
                ResultField {
                    name: "b64Url".into(),
                },
                ResultField {
                    name: "byteLength".into(),
                },
                ResultField { name: "dec".into() },
                ResultField { name: "hex".into() },
                ResultField {
                    name: "keepers".into(),
                },
                ResultField {
                    name: "prefix".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_id::Res {
            b64_std: hashmap.remove("b64Std").unwrap(),
            b64_url: hashmap.remove("b64Url").unwrap(),
            byte_length: hashmap.remove("byteLength").unwrap(),
            dec: hashmap.remove("dec").unwrap(),
            hex: hashmap.remove("hex").unwrap(),
            keepers: hashmap.remove("keepers").unwrap(),
            prefix: hashmap.remove("prefix").unwrap(),
        }
    }
}
impl random_integer::Guest for Component {
    fn invoke(name: String, args: random_integer::Args) -> random_integer::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomInteger:RandomInteger".into(),
            name,
            object: vec![
                ObjectField {
                    name: "keepers".into(),
                    value: args.keepers,
                },
                ObjectField {
                    name: "max".into(),
                    value: args.max,
                },
                ObjectField {
                    name: "min".into(),
                    value: args.min,
                },
                ObjectField {
                    name: "seed".into(),
                    value: args.seed,
                },
            ],
            results: vec![
                ResultField {
                    name: "keepers".into(),
                },
                ResultField { name: "max".into() },
                ResultField { name: "min".into() },
                ResultField {
                    name: "result".into(),
                },
                ResultField {
                    name: "seed".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_integer::Res {
            keepers: hashmap.remove("keepers").unwrap(),
            max: hashmap.remove("max").unwrap(),
            min: hashmap.remove("min").unwrap(),
            result: hashmap.remove("result").unwrap(),
            seed: hashmap.remove("seed").unwrap(),
        }
    }
}
impl random_password::Guest for Component {
    fn invoke(name: String, args: random_password::Args) -> random_password::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomPassword:RandomPassword".into(),
            name,
            object: vec![
                ObjectField {
                    name: "keepers".into(),
                    value: args.keepers,
                },
                ObjectField {
                    name: "length".into(),
                    value: args.length,
                },
                ObjectField {
                    name: "lower".into(),
                    value: args.lower,
                },
                ObjectField {
                    name: "minLower".into(),
                    value: args.min_lower,
                },
                ObjectField {
                    name: "minNumeric".into(),
                    value: args.min_numeric,
                },
                ObjectField {
                    name: "minSpecial".into(),
                    value: args.min_special,
                },
                ObjectField {
                    name: "minUpper".into(),
                    value: args.min_upper,
                },
                ObjectField {
                    name: "number".into(),
                    value: args.number,
                },
                ObjectField {
                    name: "numeric".into(),
                    value: args.numeric,
                },
                ObjectField {
                    name: "overrideSpecial".into(),
                    value: args.override_special,
                },
                ObjectField {
                    name: "special".into(),
                    value: args.special,
                },
                ObjectField {
                    name: "upper".into(),
                    value: args.upper,
                },
            ],
            results: vec![
                ResultField {
                    name: "bcryptHash".into(),
                },
                ResultField {
                    name: "keepers".into(),
                },
                ResultField {
                    name: "length".into(),
                },
                ResultField {
                    name: "lower".into(),
                },
                ResultField {
                    name: "minLower".into(),
                },
                ResultField {
                    name: "minNumeric".into(),
                },
                ResultField {
                    name: "minSpecial".into(),
                },
                ResultField {
                    name: "minUpper".into(),
                },
                ResultField {
                    name: "number".into(),
                },
                ResultField {
                    name: "numeric".into(),
                },
                ResultField {
                    name: "overrideSpecial".into(),
                },
                ResultField {
                    name: "result".into(),
                },
                ResultField {
                    name: "special".into(),
                },
                ResultField {
                    name: "upper".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

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
impl random_pet::Guest for Component {
    fn invoke(name: String, args: random_pet::Args) -> random_pet::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomPet:RandomPet".into(),
            name,
            object: vec![
                ObjectField {
                    name: "keepers".into(),
                    value: args.keepers,
                },
                ObjectField {
                    name: "length".into(),
                    value: args.length,
                },
                ObjectField {
                    name: "prefix".into(),
                    value: args.prefix,
                },
                ObjectField {
                    name: "separator".into(),
                    value: args.separator,
                },
            ],
            results: vec![
                ResultField {
                    name: "keepers".into(),
                },
                ResultField {
                    name: "length".into(),
                },
                ResultField {
                    name: "prefix".into(),
                },
                ResultField {
                    name: "separator".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_pet::Res {
            keepers: hashmap.remove("keepers").unwrap(),
            length: hashmap.remove("length").unwrap(),
            prefix: hashmap.remove("prefix").unwrap(),
            separator: hashmap.remove("separator").unwrap(),
        }
    }
}
impl random_shuffle::Guest for Component {
    fn invoke(name: String, args: random_shuffle::Args) -> random_shuffle::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomShuffle:RandomShuffle".into(),
            name,
            object: vec![
                ObjectField {
                    name: "inputs".into(),
                    value: args.inputs,
                },
                ObjectField {
                    name: "keepers".into(),
                    value: args.keepers,
                },
                ObjectField {
                    name: "resultCount".into(),
                    value: args.result_count,
                },
                ObjectField {
                    name: "seed".into(),
                    value: args.seed,
                },
            ],
            results: vec![
                ResultField {
                    name: "inputs".into(),
                },
                ResultField {
                    name: "keepers".into(),
                },
                ResultField {
                    name: "resultCount".into(),
                },
                ResultField {
                    name: "results".into(),
                },
                ResultField {
                    name: "seed".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_shuffle::Res {
            inputs: hashmap.remove("inputs").unwrap(),
            keepers: hashmap.remove("keepers").unwrap(),
            result_count: hashmap.remove("resultCount").unwrap(),
            results: hashmap.remove("results").unwrap(),
            seed: hashmap.remove("seed").unwrap(),
        }
    }
}
impl random_string::Guest for Component {
    fn invoke(name: String, args: random_string::Args) -> random_string::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomString:RandomString".into(),
            name,
            object: vec![
                ObjectField {
                    name: "keepers".into(),
                    value: args.keepers,
                },
                ObjectField {
                    name: "length".into(),
                    value: args.length,
                },
                ObjectField {
                    name: "lower".into(),
                    value: args.lower,
                },
                ObjectField {
                    name: "minLower".into(),
                    value: args.min_lower,
                },
                ObjectField {
                    name: "minNumeric".into(),
                    value: args.min_numeric,
                },
                ObjectField {
                    name: "minSpecial".into(),
                    value: args.min_special,
                },
                ObjectField {
                    name: "minUpper".into(),
                    value: args.min_upper,
                },
                ObjectField {
                    name: "number".into(),
                    value: args.number,
                },
                ObjectField {
                    name: "numeric".into(),
                    value: args.numeric,
                },
                ObjectField {
                    name: "overrideSpecial".into(),
                    value: args.override_special,
                },
                ObjectField {
                    name: "special".into(),
                    value: args.special,
                },
                ObjectField {
                    name: "upper".into(),
                    value: args.upper,
                },
            ],
            results: vec![
                ResultField {
                    name: "keepers".into(),
                },
                ResultField {
                    name: "length".into(),
                },
                ResultField {
                    name: "lower".into(),
                },
                ResultField {
                    name: "minLower".into(),
                },
                ResultField {
                    name: "minNumeric".into(),
                },
                ResultField {
                    name: "minSpecial".into(),
                },
                ResultField {
                    name: "minUpper".into(),
                },
                ResultField {
                    name: "number".into(),
                },
                ResultField {
                    name: "numeric".into(),
                },
                ResultField {
                    name: "overrideSpecial".into(),
                },
                ResultField {
                    name: "result".into(),
                },
                ResultField {
                    name: "special".into(),
                },
                ResultField {
                    name: "upper".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_string::Res {
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
impl random_uuid::Guest for Component {
    fn invoke(name: String, args: random_uuid::Args) -> random_uuid::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomUuid:RandomUuid".into(),
            name,
            object: vec![ObjectField {
                name: "keepers".into(),
                value: args.keepers,
            }],
            results: vec![
                ResultField {
                    name: "keepers".into(),
                },
                ResultField {
                    name: "result".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        random_uuid::Res {
            keepers: hashmap.remove("keepers").unwrap(),
            result: hashmap.remove("result").unwrap(),
        }
    }
}
