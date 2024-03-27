use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest};
use bindings::exports::pulumi::random::random_bytes;
use bindings::exports::pulumi::random::random_id;
use bindings::exports::pulumi::random::random_integer;
use bindings::exports::pulumi::random::random_password;
use bindings::exports::pulumi::random::random_pet;
use bindings::exports::pulumi::random::random_shuffle;
use bindings::exports::pulumi::random::random_string;
use bindings::exports::pulumi::random::random_uuid;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
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
        };

        let o = register(&request);

        random_bytes::Res {
            base64: o.get_field("base64"),
            hex: o.get_field("hex"),
            keepers: o.get_field("keepers"),
            length: o.get_field("length"),
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
        };

        let o = register(&request);

        random_id::Res {
            b64_std: o.get_field("b64Std"),
            b64_url: o.get_field("b64Url"),
            byte_length: o.get_field("byteLength"),
            dec: o.get_field("dec"),
            hex: o.get_field("hex"),
            keepers: o.get_field("keepers"),
            prefix: o.get_field("prefix"),
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
        };

        let o = register(&request);

        random_integer::Res {
            keepers: o.get_field("keepers"),
            max: o.get_field("max"),
            min: o.get_field("min"),
            result: o.get_field("result"),
            seed: o.get_field("seed"),
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
        };

        let o = register(&request);

        random_password::Res {
            bcrypt_hash: o.get_field("bcryptHash"),
            keepers: o.get_field("keepers"),
            length: o.get_field("length"),
            lower: o.get_field("lower"),
            min_lower: o.get_field("minLower"),
            min_numeric: o.get_field("minNumeric"),
            min_special: o.get_field("minSpecial"),
            min_upper: o.get_field("minUpper"),
            number: o.get_field("number"),
            numeric: o.get_field("numeric"),
            override_special: o.get_field("overrideSpecial"),
            result: o.get_field("result"),
            special: o.get_field("special"),
            upper: o.get_field("upper"),
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
        };

        let o = register(&request);

        random_pet::Res {
            keepers: o.get_field("keepers"),
            length: o.get_field("length"),
            prefix: o.get_field("prefix"),
            separator: o.get_field("separator"),
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
        };

        let o = register(&request);

        random_shuffle::Res {
            inputs: o.get_field("inputs"),
            keepers: o.get_field("keepers"),
            result_count: o.get_field("resultCount"),
            results: o.get_field("results"),
            seed: o.get_field("seed"),
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
        };

        let o = register(&request);

        random_string::Res {
            keepers: o.get_field("keepers"),
            length: o.get_field("length"),
            lower: o.get_field("lower"),
            min_lower: o.get_field("minLower"),
            min_numeric: o.get_field("minNumeric"),
            min_special: o.get_field("minSpecial"),
            min_upper: o.get_field("minUpper"),
            number: o.get_field("number"),
            numeric: o.get_field("numeric"),
            override_special: o.get_field("overrideSpecial"),
            result: o.get_field("result"),
            special: o.get_field("special"),
            upper: o.get_field("upper"),
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
        };

        let o = register(&request);

        random_uuid::Res {
            keepers: o.get_field("keepers"),
            result: o.get_field("result"),
        }

    }
}
