use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use bindings::exports::pulumi::random::random_bytes;
use bindings::exports::pulumi::random::random_id;
use bindings::exports::pulumi::random::random_integer;
use bindings::exports::pulumi::random::random_password;
use bindings::exports::pulumi::random::random_pet;
use bindings::exports::pulumi::random::random_shuffle;
use bindings::exports::pulumi::random::random_string;
use bindings::exports::pulumi::random::random_uuid;

mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl random_bytes::Guest for Component {
    fn invoke(name: String, args: random_bytes::Args) -> random_bytes::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomBytes:RandomBytes".into(),
            name,
            object: vec![
                ObjectField { name: "keepers".into(), value: args.keepers },
                ObjectField { name: "length".into(), value: args.length },
            ],
            results: vec![
                ResultField { name: "base64".into() },
                ResultField { name: "hex".into() },
                ResultField { name: "keepers".into() },
                ResultField { name: "length".into() },
            ],
        };

        let o = register(&request);

        random_bytes::Res {
            base64: o.fields.iter().find(|o| o.name == "base64").unwrap().output.duplicate(),
            hex: o.fields.iter().find(|o| o.name == "hex").unwrap().output.duplicate(),
            keepers: o.fields.iter().find(|o| o.name == "keepers").unwrap().output.duplicate(),
            length: o.fields.iter().find(|o| o.name == "length").unwrap().output.duplicate(),
        }

    }
}
impl random_id::Guest for Component {
    fn invoke(name: String, args: random_id::Args) -> random_id::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomId:RandomId".into(),
            name,
            object: vec![
                ObjectField { name: "byteLength".into(), value: args.byte_length },
                ObjectField { name: "keepers".into(), value: args.keepers },
                ObjectField { name: "prefix".into(), value: args.prefix },
            ],
            results: vec![
                ResultField { name: "b64Std".into() },
                ResultField { name: "b64Url".into() },
                ResultField { name: "byteLength".into() },
                ResultField { name: "dec".into() },
                ResultField { name: "hex".into() },
                ResultField { name: "keepers".into() },
                ResultField { name: "prefix".into() },
            ],
        };

        let o = register(&request);

        random_id::Res {
            b64_std: o.fields.iter().find(|o| o.name == "b64Std").unwrap().output.duplicate(),
            b64_url: o.fields.iter().find(|o| o.name == "b64Url").unwrap().output.duplicate(),
            byte_length: o.fields.iter().find(|o| o.name == "byteLength").unwrap().output.duplicate(),
            dec: o.fields.iter().find(|o| o.name == "dec").unwrap().output.duplicate(),
            hex: o.fields.iter().find(|o| o.name == "hex").unwrap().output.duplicate(),
            keepers: o.fields.iter().find(|o| o.name == "keepers").unwrap().output.duplicate(),
            prefix: o.fields.iter().find(|o| o.name == "prefix").unwrap().output.duplicate(),
        }

    }
}
impl random_integer::Guest for Component {
    fn invoke(name: String, args: random_integer::Args) -> random_integer::Res {
        wasm_common::setup_logger();
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

        random_integer::Res {
            keepers: o.fields.iter().find(|o| o.name == "keepers").unwrap().output.duplicate(),
            max: o.fields.iter().find(|o| o.name == "max").unwrap().output.duplicate(),
            min: o.fields.iter().find(|o| o.name == "min").unwrap().output.duplicate(),
            result: o.fields.iter().find(|o| o.name == "result").unwrap().output.duplicate(),
            seed: o.fields.iter().find(|o| o.name == "seed").unwrap().output.duplicate(),
        }

    }
}
impl random_password::Guest for Component {
    fn invoke(name: String, args: random_password::Args) -> random_password::Res {
        wasm_common::setup_logger();
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

        random_password::Res {
            bcrypt_hash: o.fields.iter().find(|o| o.name == "bcryptHash").unwrap().output.duplicate(),
            keepers: o.fields.iter().find(|o| o.name == "keepers").unwrap().output.duplicate(),
            length: o.fields.iter().find(|o| o.name == "length").unwrap().output.duplicate(),
            lower: o.fields.iter().find(|o| o.name == "lower").unwrap().output.duplicate(),
            min_lower: o.fields.iter().find(|o| o.name == "minLower").unwrap().output.duplicate(),
            min_numeric: o.fields.iter().find(|o| o.name == "minNumeric").unwrap().output.duplicate(),
            min_special: o.fields.iter().find(|o| o.name == "minSpecial").unwrap().output.duplicate(),
            min_upper: o.fields.iter().find(|o| o.name == "minUpper").unwrap().output.duplicate(),
            number: o.fields.iter().find(|o| o.name == "number").unwrap().output.duplicate(),
            numeric: o.fields.iter().find(|o| o.name == "numeric").unwrap().output.duplicate(),
            override_special: o.fields.iter().find(|o| o.name == "overrideSpecial").unwrap().output.duplicate(),
            result: o.fields.iter().find(|o| o.name == "result").unwrap().output.duplicate(),
            special: o.fields.iter().find(|o| o.name == "special").unwrap().output.duplicate(),
            upper: o.fields.iter().find(|o| o.name == "upper").unwrap().output.duplicate(),
        }

    }
}
impl random_pet::Guest for Component {
    fn invoke(name: String, args: random_pet::Args) -> random_pet::Res {
        wasm_common::setup_logger();
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

        random_pet::Res {
            keepers: o.fields.iter().find(|o| o.name == "keepers").unwrap().output.duplicate(),
            length: o.fields.iter().find(|o| o.name == "length").unwrap().output.duplicate(),
            prefix: o.fields.iter().find(|o| o.name == "prefix").unwrap().output.duplicate(),
            separator: o.fields.iter().find(|o| o.name == "separator").unwrap().output.duplicate(),
        }

    }
}
impl random_shuffle::Guest for Component {
    fn invoke(name: String, args: random_shuffle::Args) -> random_shuffle::Res {
        wasm_common::setup_logger();
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

        random_shuffle::Res {
            inputs: o.fields.iter().find(|o| o.name == "inputs").unwrap().output.duplicate(),
            keepers: o.fields.iter().find(|o| o.name == "keepers").unwrap().output.duplicate(),
            result_count: o.fields.iter().find(|o| o.name == "resultCount").unwrap().output.duplicate(),
            results: o.fields.iter().find(|o| o.name == "results").unwrap().output.duplicate(),
            seed: o.fields.iter().find(|o| o.name == "seed").unwrap().output.duplicate(),
        }

    }
}
impl random_string::Guest for Component {
    fn invoke(name: String, args: random_string::Args) -> random_string::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomString:RandomString".into(),
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

        random_string::Res {
            keepers: o.fields.iter().find(|o| o.name == "keepers").unwrap().output.duplicate(),
            length: o.fields.iter().find(|o| o.name == "length").unwrap().output.duplicate(),
            lower: o.fields.iter().find(|o| o.name == "lower").unwrap().output.duplicate(),
            min_lower: o.fields.iter().find(|o| o.name == "minLower").unwrap().output.duplicate(),
            min_numeric: o.fields.iter().find(|o| o.name == "minNumeric").unwrap().output.duplicate(),
            min_special: o.fields.iter().find(|o| o.name == "minSpecial").unwrap().output.duplicate(),
            min_upper: o.fields.iter().find(|o| o.name == "minUpper").unwrap().output.duplicate(),
            number: o.fields.iter().find(|o| o.name == "number").unwrap().output.duplicate(),
            numeric: o.fields.iter().find(|o| o.name == "numeric").unwrap().output.duplicate(),
            override_special: o.fields.iter().find(|o| o.name == "overrideSpecial").unwrap().output.duplicate(),
            result: o.fields.iter().find(|o| o.name == "result").unwrap().output.duplicate(),
            special: o.fields.iter().find(|o| o.name == "special").unwrap().output.duplicate(),
            upper: o.fields.iter().find(|o| o.name == "upper").unwrap().output.duplicate(),
        }

    }
}
impl random_uuid::Guest for Component {
    fn invoke(name: String, args: random_uuid::Args) -> random_uuid::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "random:index/randomUuid:RandomUuid".into(),
            name,
            object: vec![
                ObjectField { name: "keepers".into(), value: args.keepers },
            ],
            results: vec![
                ResultField { name: "keepers".into() },
                ResultField { name: "result".into() },
            ],
        };

        let o = register(&request);

        random_uuid::Res {
            keepers: o.fields.iter().find(|o| o.name == "keepers").unwrap().output.duplicate(),
            result: o.fields.iter().find(|o| o.name == "result").unwrap().output.duplicate(),
        }

    }
}
