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
                ResultField { name: "base64".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "hex".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "keepers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "length".into(), schema: vec![163, 73, 110, 116] },
            ],
        };

        let o = register(&request);

        random_bytes::Res {
            base64: o.get_field("base64", true),
            hex: o.get_field("hex", true),
            keepers: o.get_field("keepers", false),
            length: o.get_field("length", true),
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
                ResultField { name: "b64Std".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "b64Url".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "byteLength".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "dec".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "hex".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "keepers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "prefix".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        random_id::Res {
            b64_std: o.get_field("b64Std", true),
            b64_url: o.get_field("b64Url", true),
            byte_length: o.get_field("byteLength", true),
            dec: o.get_field("dec", true),
            hex: o.get_field("hex", true),
            keepers: o.get_field("keepers", false),
            prefix: o.get_field("prefix", false),
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
                ResultField { name: "keepers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "max".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "min".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "result".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "seed".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        random_integer::Res {
            keepers: o.get_field("keepers", false),
            max: o.get_field("max", true),
            min: o.get_field("min", true),
            result: o.get_field("result", true),
            seed: o.get_field("seed", false),
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
                ResultField { name: "bcryptHash".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "keepers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "length".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "lower".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "minLower".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "minNumeric".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "minSpecial".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "minUpper".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "number".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "numeric".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "overrideSpecial".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "result".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "special".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "upper".into(), schema: vec![164, 66, 111, 111, 108] },
            ],
        };

        let o = register(&request);

        random_password::Res {
            bcrypt_hash: o.get_field("bcryptHash", true),
            keepers: o.get_field("keepers", false),
            length: o.get_field("length", true),
            lower: o.get_field("lower", true),
            min_lower: o.get_field("minLower", true),
            min_numeric: o.get_field("minNumeric", true),
            min_special: o.get_field("minSpecial", true),
            min_upper: o.get_field("minUpper", true),
            number: o.get_field("number", true),
            numeric: o.get_field("numeric", true),
            override_special: o.get_field("overrideSpecial", false),
            result: o.get_field("result", true),
            special: o.get_field("special", true),
            upper: o.get_field("upper", true),
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
                ResultField { name: "keepers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "length".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "prefix".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "separator".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        random_pet::Res {
            keepers: o.get_field("keepers", false),
            length: o.get_field("length", true),
            prefix: o.get_field("prefix", false),
            separator: o.get_field("separator", true),
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
                ResultField { name: "inputs".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "keepers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "resultCount".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116] },
                ResultField { name: "results".into(), schema: vec![129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "seed".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        random_shuffle::Res {
            inputs: o.get_field("inputs", true),
            keepers: o.get_field("keepers", false),
            result_count: o.get_field("resultCount", false),
            results: o.get_field("results", true),
            seed: o.get_field("seed", false),
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
                ResultField { name: "keepers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "length".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "lower".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "minLower".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "minNumeric".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "minSpecial".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "minUpper".into(), schema: vec![163, 73, 110, 116] },
                ResultField { name: "number".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "numeric".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "overrideSpecial".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "result".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "special".into(), schema: vec![164, 66, 111, 111, 108] },
                ResultField { name: "upper".into(), schema: vec![164, 66, 111, 111, 108] },
            ],
        };

        let o = register(&request);

        random_string::Res {
            keepers: o.get_field("keepers", false),
            length: o.get_field("length", true),
            lower: o.get_field("lower", true),
            min_lower: o.get_field("minLower", true),
            min_numeric: o.get_field("minNumeric", true),
            min_special: o.get_field("minSpecial", true),
            min_upper: o.get_field("minUpper", true),
            number: o.get_field("number", true),
            numeric: o.get_field("numeric", true),
            override_special: o.get_field("overrideSpecial", false),
            result: o.get_field("result", true),
            special: o.get_field("special", true),
            upper: o.get_field("upper", true),
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
                ResultField { name: "keepers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "result".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        random_uuid::Res {
            keepers: o.get_field("keepers", false),
            result: o.get_field("result", true),
        }

    }
}
