
pub mod random_bytes {

    pub struct RandomBytesArgs<'a> {
        pub name: &'a str,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
    }

    pub struct RandomBytesResult {
        pub base64: pulumi_wasm_rust::Output<Option<String>>,
        pub hex: pulumi_wasm_rust::Output<Option<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
    }

    pub fn random_bytes(args: RandomBytesArgs) -> RandomBytesResult {

        let result = crate::bindings::pulumi::random::random_bytes::invoke(args.name, &crate::bindings::pulumi::random::random_bytes::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            length: &crate::clone::<i32>(args.length),
        });

        RandomBytesResult {
            base64: crate::random_to_domain_mapper::<Option<String>>(result.base64),
            hex: crate::random_to_domain_mapper::<Option<String>>(result.hex),
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            length: crate::random_to_domain_mapper::<i32>(result.length),
        }
    }

}


pub mod random_id {

    pub struct RandomIdArgs<'a> {
        pub name: &'a str,
        pub byte_length: pulumi_wasm_rust::Output<i32>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomIdResult {
        pub b64_std: pulumi_wasm_rust::Output<Option<String>>,
        pub b64_url: pulumi_wasm_rust::Output<Option<String>>,
        pub byte_length: pulumi_wasm_rust::Output<i32>,
        pub dec: pulumi_wasm_rust::Output<Option<String>>,
        pub hex: pulumi_wasm_rust::Output<Option<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn random_id(args: RandomIdArgs) -> RandomIdResult {

        let result = crate::bindings::pulumi::random::random_id::invoke(args.name, &crate::bindings::pulumi::random::random_id::Args {
            byte_length: &crate::clone::<i32>(args.byte_length),
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            prefix: &crate::clone::<Option<String>>(args.prefix),
        });

        RandomIdResult {
            b64_std: crate::random_to_domain_mapper::<Option<String>>(result.b64_std),
            b64_url: crate::random_to_domain_mapper::<Option<String>>(result.b64_url),
            byte_length: crate::random_to_domain_mapper::<i32>(result.byte_length),
            dec: crate::random_to_domain_mapper::<Option<String>>(result.dec),
            hex: crate::random_to_domain_mapper::<Option<String>>(result.hex),
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            prefix: crate::random_to_domain_mapper::<Option<String>>(result.prefix),
        }
    }

}


pub mod random_integer {

    pub struct RandomIntegerArgs<'a> {
        pub name: &'a str,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub max: pulumi_wasm_rust::Output<i32>,
        pub min: pulumi_wasm_rust::Output<i32>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomIntegerResult {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub max: pulumi_wasm_rust::Output<i32>,
        pub min: pulumi_wasm_rust::Output<i32>,
        pub result: pulumi_wasm_rust::Output<Option<i32>>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn random_integer(args: RandomIntegerArgs) -> RandomIntegerResult {

        let result = crate::bindings::pulumi::random::random_integer::invoke(args.name, &crate::bindings::pulumi::random::random_integer::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            max: &crate::clone::<i32>(args.max),
            min: &crate::clone::<i32>(args.min),
            seed: &crate::clone::<Option<String>>(args.seed),
        });

        RandomIntegerResult {
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            max: crate::random_to_domain_mapper::<i32>(result.max),
            min: crate::random_to_domain_mapper::<i32>(result.min),
            result: crate::random_to_domain_mapper::<Option<i32>>(result.result),
            seed: crate::random_to_domain_mapper::<Option<String>>(result.seed),
        }
    }

}


pub mod random_password {

    pub struct RandomPasswordArgs<'a> {
        pub name: &'a str,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub struct RandomPasswordResult {
        pub bcrypt_hash: pulumi_wasm_rust::Output<Option<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        pub result: pulumi_wasm_rust::Output<Option<String>>,
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub fn random_password(args: RandomPasswordArgs) -> RandomPasswordResult {

        let result = crate::bindings::pulumi::random::random_password::invoke(args.name, &crate::bindings::pulumi::random::random_password::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            length: &crate::clone::<i32>(args.length),
            lower: &crate::clone::<Option<bool>>(args.lower),
            min_lower: &crate::clone::<Option<i32>>(args.min_lower),
            min_numeric: &crate::clone::<Option<i32>>(args.min_numeric),
            min_special: &crate::clone::<Option<i32>>(args.min_special),
            min_upper: &crate::clone::<Option<i32>>(args.min_upper),
            number: &crate::clone::<Option<bool>>(args.number),
            numeric: &crate::clone::<Option<bool>>(args.numeric),
            override_special: &crate::clone::<Option<String>>(args.override_special),
            special: &crate::clone::<Option<bool>>(args.special),
            upper: &crate::clone::<Option<bool>>(args.upper),
        });

        RandomPasswordResult {
            bcrypt_hash: crate::random_to_domain_mapper::<Option<String>>(result.bcrypt_hash),
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            length: crate::random_to_domain_mapper::<i32>(result.length),
            lower: crate::random_to_domain_mapper::<Option<bool>>(result.lower),
            min_lower: crate::random_to_domain_mapper::<Option<i32>>(result.min_lower),
            min_numeric: crate::random_to_domain_mapper::<Option<i32>>(result.min_numeric),
            min_special: crate::random_to_domain_mapper::<Option<i32>>(result.min_special),
            min_upper: crate::random_to_domain_mapper::<Option<i32>>(result.min_upper),
            number: crate::random_to_domain_mapper::<Option<bool>>(result.number),
            numeric: crate::random_to_domain_mapper::<Option<bool>>(result.numeric),
            override_special: crate::random_to_domain_mapper::<Option<String>>(result.override_special),
            result: crate::random_to_domain_mapper::<Option<String>>(result.result),
            special: crate::random_to_domain_mapper::<Option<bool>>(result.special),
            upper: crate::random_to_domain_mapper::<Option<bool>>(result.upper),
        }
    }

}


pub mod random_pet {

    pub struct RandomPetArgs<'a> {
        pub name: &'a str,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<Option<i32>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
        pub separator: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomPetResult {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<Option<i32>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
        pub separator: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn random_pet(args: RandomPetArgs) -> RandomPetResult {

        let result = crate::bindings::pulumi::random::random_pet::invoke(args.name, &crate::bindings::pulumi::random::random_pet::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            length: &crate::clone::<Option<i32>>(args.length),
            prefix: &crate::clone::<Option<String>>(args.prefix),
            separator: &crate::clone::<Option<String>>(args.separator),
        });

        RandomPetResult {
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            length: crate::random_to_domain_mapper::<Option<i32>>(result.length),
            prefix: crate::random_to_domain_mapper::<Option<String>>(result.prefix),
            separator: crate::random_to_domain_mapper::<Option<String>>(result.separator),
        }
    }

}


pub mod random_shuffle {

    pub struct RandomShuffleArgs<'a> {
        pub name: &'a str,
        pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomShuffleResult {
        pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
        pub results: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn random_shuffle(args: RandomShuffleArgs) -> RandomShuffleResult {

        let result = crate::bindings::pulumi::random::random_shuffle::invoke(args.name, &crate::bindings::pulumi::random::random_shuffle::Args {
            inputs: &crate::clone::<Vec<String>>(args.inputs),
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            result_count: &crate::clone::<Option<i32>>(args.result_count),
            seed: &crate::clone::<Option<String>>(args.seed),
        });

        RandomShuffleResult {
            inputs: crate::random_to_domain_mapper::<Vec<String>>(result.inputs),
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            result_count: crate::random_to_domain_mapper::<Option<i32>>(result.result_count),
            results: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.results),
            seed: crate::random_to_domain_mapper::<Option<String>>(result.seed),
        }
    }

}


pub mod random_string {

    pub struct RandomStringArgs<'a> {
        pub name: &'a str,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub struct RandomStringResult {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        pub result: pulumi_wasm_rust::Output<Option<String>>,
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub fn random_string(args: RandomStringArgs) -> RandomStringResult {

        let result = crate::bindings::pulumi::random::random_string::invoke(args.name, &crate::bindings::pulumi::random::random_string::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            length: &crate::clone::<i32>(args.length),
            lower: &crate::clone::<Option<bool>>(args.lower),
            min_lower: &crate::clone::<Option<i32>>(args.min_lower),
            min_numeric: &crate::clone::<Option<i32>>(args.min_numeric),
            min_special: &crate::clone::<Option<i32>>(args.min_special),
            min_upper: &crate::clone::<Option<i32>>(args.min_upper),
            number: &crate::clone::<Option<bool>>(args.number),
            numeric: &crate::clone::<Option<bool>>(args.numeric),
            override_special: &crate::clone::<Option<String>>(args.override_special),
            special: &crate::clone::<Option<bool>>(args.special),
            upper: &crate::clone::<Option<bool>>(args.upper),
        });

        RandomStringResult {
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            length: crate::random_to_domain_mapper::<i32>(result.length),
            lower: crate::random_to_domain_mapper::<Option<bool>>(result.lower),
            min_lower: crate::random_to_domain_mapper::<Option<i32>>(result.min_lower),
            min_numeric: crate::random_to_domain_mapper::<Option<i32>>(result.min_numeric),
            min_special: crate::random_to_domain_mapper::<Option<i32>>(result.min_special),
            min_upper: crate::random_to_domain_mapper::<Option<i32>>(result.min_upper),
            number: crate::random_to_domain_mapper::<Option<bool>>(result.number),
            numeric: crate::random_to_domain_mapper::<Option<bool>>(result.numeric),
            override_special: crate::random_to_domain_mapper::<Option<String>>(result.override_special),
            result: crate::random_to_domain_mapper::<Option<String>>(result.result),
            special: crate::random_to_domain_mapper::<Option<bool>>(result.special),
            upper: crate::random_to_domain_mapper::<Option<bool>>(result.upper),
        }
    }

}


pub mod random_uuid {

    pub struct RandomUuidArgs<'a> {
        pub name: &'a str,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub struct RandomUuidResult {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub result: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn random_uuid(args: RandomUuidArgs) -> RandomUuidResult {

        let result = crate::bindings::pulumi::random::random_uuid::invoke(args.name, &crate::bindings::pulumi::random::random_uuid::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
        });

        RandomUuidResult {
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            result: crate::random_to_domain_mapper::<Option<String>>(result.result),
        }
    }

}
