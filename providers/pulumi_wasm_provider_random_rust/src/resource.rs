pub mod random_bytes {

    pub struct RandomBytesArgs {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
    }

    pub struct RandomBytesResult {
        pub base64: pulumi_wasm_rust::Output<String>,
        pub hex: pulumi_wasm_rust::Output<String>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
    }

    pub fn random_bytes(name: &str, args: RandomBytesArgs) -> RandomBytesResult {
        let result = crate::bindings::pulumi::random::random_bytes::invoke(
            name,
            &crate::bindings::pulumi::random::random_bytes::Args {
                keepers: args.keepers.get_inner(),
                length: args.length.get_inner(),
            },
        );

        RandomBytesResult {
            base64: crate::into_domain(result.base64),
            hex: crate::into_domain(result.hex),
            keepers: crate::into_domain(result.keepers),
            length: crate::into_domain(result.length),
        }
    }
}

pub mod random_id {

    pub struct RandomIdArgs {
        pub byte_length: pulumi_wasm_rust::Output<i32>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomIdResult {
        pub b64_std: pulumi_wasm_rust::Output<String>,
        pub b64_url: pulumi_wasm_rust::Output<String>,
        pub byte_length: pulumi_wasm_rust::Output<i32>,
        pub dec: pulumi_wasm_rust::Output<String>,
        pub hex: pulumi_wasm_rust::Output<String>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn random_id(name: &str, args: RandomIdArgs) -> RandomIdResult {
        let result = crate::bindings::pulumi::random::random_id::invoke(
            name,
            &crate::bindings::pulumi::random::random_id::Args {
                byte_length: args.byte_length.get_inner(),
                keepers: args.keepers.get_inner(),
                prefix: args.prefix.get_inner(),
            },
        );

        RandomIdResult {
            b64_std: crate::into_domain(result.b64_std),
            b64_url: crate::into_domain(result.b64_url),
            byte_length: crate::into_domain(result.byte_length),
            dec: crate::into_domain(result.dec),
            hex: crate::into_domain(result.hex),
            keepers: crate::into_domain(result.keepers),
            prefix: crate::into_domain(result.prefix),
        }
    }
}

pub mod random_integer {

    pub struct RandomIntegerArgs {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub max: pulumi_wasm_rust::Output<i32>,
        pub min: pulumi_wasm_rust::Output<i32>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomIntegerResult {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub max: pulumi_wasm_rust::Output<i32>,
        pub min: pulumi_wasm_rust::Output<i32>,
        pub result: pulumi_wasm_rust::Output<i32>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn random_integer(name: &str, args: RandomIntegerArgs) -> RandomIntegerResult {
        let result = crate::bindings::pulumi::random::random_integer::invoke(
            name,
            &crate::bindings::pulumi::random::random_integer::Args {
                keepers: args.keepers.get_inner(),
                max: args.max.get_inner(),
                min: args.min.get_inner(),
                seed: args.seed.get_inner(),
            },
        );

        RandomIntegerResult {
            keepers: crate::into_domain(result.keepers),
            max: crate::into_domain(result.max),
            min: crate::into_domain(result.min),
            result: crate::into_domain(result.result),
            seed: crate::into_domain(result.seed),
        }
    }
}

pub mod random_password {

    pub struct RandomPasswordArgs {
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
        pub bcrypt_hash: pulumi_wasm_rust::Output<String>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<bool>,
        pub min_lower: pulumi_wasm_rust::Output<i32>,
        pub min_numeric: pulumi_wasm_rust::Output<i32>,
        pub min_special: pulumi_wasm_rust::Output<i32>,
        pub min_upper: pulumi_wasm_rust::Output<i32>,
        pub number: pulumi_wasm_rust::Output<bool>,
        pub numeric: pulumi_wasm_rust::Output<bool>,
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        pub result: pulumi_wasm_rust::Output<String>,
        pub special: pulumi_wasm_rust::Output<bool>,
        pub upper: pulumi_wasm_rust::Output<bool>,
    }

    pub fn random_password(name: &str, args: RandomPasswordArgs) -> RandomPasswordResult {
        let result = crate::bindings::pulumi::random::random_password::invoke(
            name,
            &crate::bindings::pulumi::random::random_password::Args {
                keepers: args.keepers.get_inner(),
                length: args.length.get_inner(),
                lower: args.lower.get_inner(),
                min_lower: args.min_lower.get_inner(),
                min_numeric: args.min_numeric.get_inner(),
                min_special: args.min_special.get_inner(),
                min_upper: args.min_upper.get_inner(),
                number: args.number.get_inner(),
                numeric: args.numeric.get_inner(),
                override_special: args.override_special.get_inner(),
                special: args.special.get_inner(),
                upper: args.upper.get_inner(),
            },
        );

        RandomPasswordResult {
            bcrypt_hash: crate::into_domain(result.bcrypt_hash),
            keepers: crate::into_domain(result.keepers),
            length: crate::into_domain(result.length),
            lower: crate::into_domain(result.lower),
            min_lower: crate::into_domain(result.min_lower),
            min_numeric: crate::into_domain(result.min_numeric),
            min_special: crate::into_domain(result.min_special),
            min_upper: crate::into_domain(result.min_upper),
            number: crate::into_domain(result.number),
            numeric: crate::into_domain(result.numeric),
            override_special: crate::into_domain(result.override_special),
            result: crate::into_domain(result.result),
            special: crate::into_domain(result.special),
            upper: crate::into_domain(result.upper),
        }
    }
}

pub mod random_pet {

    pub struct RandomPetArgs {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<Option<i32>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
        pub separator: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomPetResult {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
        pub separator: pulumi_wasm_rust::Output<String>,
    }

    pub fn random_pet(name: &str, args: RandomPetArgs) -> RandomPetResult {
        let result = crate::bindings::pulumi::random::random_pet::invoke(
            name,
            &crate::bindings::pulumi::random::random_pet::Args {
                keepers: args.keepers.get_inner(),
                length: args.length.get_inner(),
                prefix: args.prefix.get_inner(),
                separator: args.separator.get_inner(),
            },
        );

        RandomPetResult {
            keepers: crate::into_domain(result.keepers),
            length: crate::into_domain(result.length),
            prefix: crate::into_domain(result.prefix),
            separator: crate::into_domain(result.separator),
        }
    }
}

pub mod random_shuffle {

    pub struct RandomShuffleArgs {
        pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomShuffleResult {
        pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
        pub results: pulumi_wasm_rust::Output<Vec<String>>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn random_shuffle(name: &str, args: RandomShuffleArgs) -> RandomShuffleResult {
        let result = crate::bindings::pulumi::random::random_shuffle::invoke(
            name,
            &crate::bindings::pulumi::random::random_shuffle::Args {
                inputs: args.inputs.get_inner(),
                keepers: args.keepers.get_inner(),
                result_count: args.result_count.get_inner(),
                seed: args.seed.get_inner(),
            },
        );

        RandomShuffleResult {
            inputs: crate::into_domain(result.inputs),
            keepers: crate::into_domain(result.keepers),
            result_count: crate::into_domain(result.result_count),
            results: crate::into_domain(result.results),
            seed: crate::into_domain(result.seed),
        }
    }
}

pub mod random_string {

    pub struct RandomStringArgs {
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
        pub lower: pulumi_wasm_rust::Output<bool>,
        pub min_lower: pulumi_wasm_rust::Output<i32>,
        pub min_numeric: pulumi_wasm_rust::Output<i32>,
        pub min_special: pulumi_wasm_rust::Output<i32>,
        pub min_upper: pulumi_wasm_rust::Output<i32>,
        pub number: pulumi_wasm_rust::Output<bool>,
        pub numeric: pulumi_wasm_rust::Output<bool>,
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        pub result: pulumi_wasm_rust::Output<String>,
        pub special: pulumi_wasm_rust::Output<bool>,
        pub upper: pulumi_wasm_rust::Output<bool>,
    }

    pub fn random_string(name: &str, args: RandomStringArgs) -> RandomStringResult {
        let result = crate::bindings::pulumi::random::random_string::invoke(
            name,
            &crate::bindings::pulumi::random::random_string::Args {
                keepers: args.keepers.get_inner(),
                length: args.length.get_inner(),
                lower: args.lower.get_inner(),
                min_lower: args.min_lower.get_inner(),
                min_numeric: args.min_numeric.get_inner(),
                min_special: args.min_special.get_inner(),
                min_upper: args.min_upper.get_inner(),
                number: args.number.get_inner(),
                numeric: args.numeric.get_inner(),
                override_special: args.override_special.get_inner(),
                special: args.special.get_inner(),
                upper: args.upper.get_inner(),
            },
        );

        RandomStringResult {
            keepers: crate::into_domain(result.keepers),
            length: crate::into_domain(result.length),
            lower: crate::into_domain(result.lower),
            min_lower: crate::into_domain(result.min_lower),
            min_numeric: crate::into_domain(result.min_numeric),
            min_special: crate::into_domain(result.min_special),
            min_upper: crate::into_domain(result.min_upper),
            number: crate::into_domain(result.number),
            numeric: crate::into_domain(result.numeric),
            override_special: crate::into_domain(result.override_special),
            result: crate::into_domain(result.result),
            special: crate::into_domain(result.special),
            upper: crate::into_domain(result.upper),
        }
    }
}

pub mod random_uuid {

    pub struct RandomUuidArgs {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub struct RandomUuidResult {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub result: pulumi_wasm_rust::Output<String>,
    }

    pub fn random_uuid(name: &str, args: RandomUuidArgs) -> RandomUuidResult {
        let result = crate::bindings::pulumi::random::random_uuid::invoke(
            name,
            &crate::bindings::pulumi::random::random_uuid::Args {
                keepers: args.keepers.get_inner(),
            },
        );

        RandomUuidResult {
            keepers: crate::into_domain(result.keepers),
            result: crate::into_domain(result.result),
        }
    }
}
