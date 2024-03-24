use crate::bindings::component::pulumi_wasm::register_interface::RegisterResourceRequest;
use crate::bindings::component::pulumi_wasm::register_interface::ObjectField;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl bindings::exports::pulumi::random::random_index_random_bytes_random_bytes::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::random::random_index_random_bytes_random_bytes::Args) -> bindings::exports::pulumi::random::random_index_random_bytes_random_bytes::Res {
/*        let mut res = bindings::exports::pulumi::random::random_index_random_bytes_random_bytes::res {
        };
        res
*/

//        let r#type = "random:index/randomBytes:RandomBytes".to_string();

        let request = RegisterResourceRequest {
            type_: "random:index/randomBytes:RandomBytes".into(),
            name,
            object: vec![
                ObjectField { name: "keepers".into(), value: args.keepers },
                ObjectField { name: "length".into(), value: args.length },
            ],
        };

        todo!()

    }
}
impl bindings::exports::pulumi::random::random_index_random_id_random_id::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::random::random_index_random_id_random_id::Args) -> bindings::exports::pulumi::random::random_index_random_id_random_id::Res {
/*        let mut res = bindings::exports::pulumi::random::random_index_random_id_random_id::res {
        };
        res
*/

//        let r#type = "random:index/randomId:RandomId".to_string();

        let request = RegisterResourceRequest {
            type_: "random:index/randomId:RandomId".into(),
            name,
            object: vec![
                ObjectField { name: "byteLength".into(), value: args.byte_length },
                ObjectField { name: "keepers".into(), value: args.keepers },
                ObjectField { name: "prefix".into(), value: args.prefix },
            ],
        };

        todo!()

    }
}
impl bindings::exports::pulumi::random::random_index_random_integer_random_integer::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::random::random_index_random_integer_random_integer::Args) -> bindings::exports::pulumi::random::random_index_random_integer_random_integer::Res {
/*        let mut res = bindings::exports::pulumi::random::random_index_random_integer_random_integer::res {
        };
        res
*/

//        let r#type = "random:index/randomInteger:RandomInteger".to_string();

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

        todo!()

    }
}
impl bindings::exports::pulumi::random::random_index_random_password_random_password::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::random::random_index_random_password_random_password::Args) -> bindings::exports::pulumi::random::random_index_random_password_random_password::Res {
/*        let mut res = bindings::exports::pulumi::random::random_index_random_password_random_password::res {
        };
        res
*/

//        let r#type = "random:index/randomPassword:RandomPassword".to_string();

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

        todo!()

    }
}
impl bindings::exports::pulumi::random::random_index_random_pet_random_pet::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::random::random_index_random_pet_random_pet::Args) -> bindings::exports::pulumi::random::random_index_random_pet_random_pet::Res {
/*        let mut res = bindings::exports::pulumi::random::random_index_random_pet_random_pet::res {
        };
        res
*/

//        let r#type = "random:index/randomPet:RandomPet".to_string();

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

        todo!()

    }
}
impl bindings::exports::pulumi::random::random_index_random_shuffle_random_shuffle::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::random::random_index_random_shuffle_random_shuffle::Args) -> bindings::exports::pulumi::random::random_index_random_shuffle_random_shuffle::Res {
/*        let mut res = bindings::exports::pulumi::random::random_index_random_shuffle_random_shuffle::res {
        };
        res
*/

//        let r#type = "random:index/randomShuffle:RandomShuffle".to_string();

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

        todo!()

    }
}
impl bindings::exports::pulumi::random::random_index_random_string_random_string::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::random::random_index_random_string_random_string::Args) -> bindings::exports::pulumi::random::random_index_random_string_random_string::Res {
/*        let mut res = bindings::exports::pulumi::random::random_index_random_string_random_string::res {
        };
        res
*/

//        let r#type = "random:index/randomString:RandomString".to_string();

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

        todo!()

    }
}
impl bindings::exports::pulumi::random::random_index_random_uuid_random_uuid::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::random::random_index_random_uuid_random_uuid::Args) -> bindings::exports::pulumi::random::random_index_random_uuid_random_uuid::Res {
/*        let mut res = bindings::exports::pulumi::random::random_index_random_uuid_random_uuid::res {
        };
        res
*/

//        let r#type = "random:index/randomUuid:RandomUuid".to_string();

        let request = RegisterResourceRequest {
            type_: "random:index/randomUuid:RandomUuid".into(),
            name,
            object: vec![
                ObjectField { name: "keepers".into(), value: args.keepers },
            ],
        };

        todo!()

    }
}
