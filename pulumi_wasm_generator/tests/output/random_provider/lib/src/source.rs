
mod random_bytes {

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

    fn random_bytes<'a>(args: RandomBytesArgs<'a>) -> RandomBytesResult {

        let result = crate::bindings::pulumi::random::random_bytes::invoke(args.name, &crate::bindings::pulumi::random::random_bytes::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            length: &crate::clone::<i32>(args.length),
        });


        let result = RandomBytesResult {
            base64: crate::random_to_domain_mapper::<Option<String>>(result.base64),
            hex: crate::random_to_domain_mapper::<Option<String>>(result.hex),
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            length: crate::random_to_domain_mapper::<i32>(result.length),
        };

        result
    }

}


mod random_id {

    pub struct RandomIdArgs<'a> {
        pub name: &'a str,
        pub byteLength: pulumi_wasm_rust::Output<i32>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomIdResult {
        pub b64Std: pulumi_wasm_rust::Output<Option<String>>,
        pub b64Url: pulumi_wasm_rust::Output<Option<String>>,
        pub byteLength: pulumi_wasm_rust::Output<i32>,
        pub dec: pulumi_wasm_rust::Output<Option<String>>,
        pub hex: pulumi_wasm_rust::Output<Option<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    }

    fn random_id<'a>(args: RandomIdArgs<'a>) -> RandomIdResult {

        let result = crate::bindings::pulumi::random::random_id::invoke(args.name, &crate::bindings::pulumi::random::random_id::Args {
            byte_length: &crate::clone::<i32>(args.byteLength),
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            prefix: &crate::clone::<Option<String>>(args.prefix),
        });


        let result = RandomIdResult {
            b64Std: crate::random_to_domain_mapper::<Option<String>>(result.b64_std),
            b64Url: crate::random_to_domain_mapper::<Option<String>>(result.b64_url),
            byteLength: crate::random_to_domain_mapper::<i32>(result.byte_length),
            dec: crate::random_to_domain_mapper::<Option<String>>(result.dec),
            hex: crate::random_to_domain_mapper::<Option<String>>(result.hex),
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            prefix: crate::random_to_domain_mapper::<Option<String>>(result.prefix),
        };

        result
    }

}


mod random_integer {

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

    fn random_integer<'a>(args: RandomIntegerArgs<'a>) -> RandomIntegerResult {

        let result = crate::bindings::pulumi::random::random_integer::invoke(args.name, &crate::bindings::pulumi::random::random_integer::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            max: &crate::clone::<i32>(args.max),
            min: &crate::clone::<i32>(args.min),
            seed: &crate::clone::<Option<String>>(args.seed),
        });


        let result = RandomIntegerResult {
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            max: crate::random_to_domain_mapper::<i32>(result.max),
            min: crate::random_to_domain_mapper::<i32>(result.min),
            result: crate::random_to_domain_mapper::<Option<i32>>(result.result),
            seed: crate::random_to_domain_mapper::<Option<String>>(result.seed),
        };

        result
    }

}


mod random_password {

    pub struct RandomPasswordArgs<'a> {
        pub name: &'a str,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        pub minLower: pulumi_wasm_rust::Output<Option<i32>>,
        pub minNumeric: pulumi_wasm_rust::Output<Option<i32>>,
        pub minSpecial: pulumi_wasm_rust::Output<Option<i32>>,
        pub minUpper: pulumi_wasm_rust::Output<Option<i32>>,
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
        pub overrideSpecial: pulumi_wasm_rust::Output<Option<String>>,
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub struct RandomPasswordResult {
        pub bcryptHash: pulumi_wasm_rust::Output<Option<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        pub minLower: pulumi_wasm_rust::Output<Option<i32>>,
        pub minNumeric: pulumi_wasm_rust::Output<Option<i32>>,
        pub minSpecial: pulumi_wasm_rust::Output<Option<i32>>,
        pub minUpper: pulumi_wasm_rust::Output<Option<i32>>,
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
        pub overrideSpecial: pulumi_wasm_rust::Output<Option<String>>,
        pub result: pulumi_wasm_rust::Output<Option<String>>,
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }

    fn random_password<'a>(args: RandomPasswordArgs<'a>) -> RandomPasswordResult {

        let result = crate::bindings::pulumi::random::random_password::invoke(args.name, &crate::bindings::pulumi::random::random_password::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            length: &crate::clone::<i32>(args.length),
            lower: &crate::clone::<Option<bool>>(args.lower),
            min_lower: &crate::clone::<Option<i32>>(args.minLower),
            min_numeric: &crate::clone::<Option<i32>>(args.minNumeric),
            min_special: &crate::clone::<Option<i32>>(args.minSpecial),
            min_upper: &crate::clone::<Option<i32>>(args.minUpper),
            number: &crate::clone::<Option<bool>>(args.number),
            numeric: &crate::clone::<Option<bool>>(args.numeric),
            override_special: &crate::clone::<Option<String>>(args.overrideSpecial),
            special: &crate::clone::<Option<bool>>(args.special),
            upper: &crate::clone::<Option<bool>>(args.upper),
        });


        let result = RandomPasswordResult {
            bcryptHash: crate::random_to_domain_mapper::<Option<String>>(result.bcrypt_hash),
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            length: crate::random_to_domain_mapper::<i32>(result.length),
            lower: crate::random_to_domain_mapper::<Option<bool>>(result.lower),
            minLower: crate::random_to_domain_mapper::<Option<i32>>(result.min_lower),
            minNumeric: crate::random_to_domain_mapper::<Option<i32>>(result.min_numeric),
            minSpecial: crate::random_to_domain_mapper::<Option<i32>>(result.min_special),
            minUpper: crate::random_to_domain_mapper::<Option<i32>>(result.min_upper),
            number: crate::random_to_domain_mapper::<Option<bool>>(result.number),
            numeric: crate::random_to_domain_mapper::<Option<bool>>(result.numeric),
            overrideSpecial: crate::random_to_domain_mapper::<Option<String>>(result.override_special),
            result: crate::random_to_domain_mapper::<Option<String>>(result.result),
            special: crate::random_to_domain_mapper::<Option<bool>>(result.special),
            upper: crate::random_to_domain_mapper::<Option<bool>>(result.upper),
        };

        result
    }

}


mod random_pet {

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

    fn random_pet<'a>(args: RandomPetArgs<'a>) -> RandomPetResult {

        let result = crate::bindings::pulumi::random::random_pet::invoke(args.name, &crate::bindings::pulumi::random::random_pet::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            length: &crate::clone::<Option<i32>>(args.length),
            prefix: &crate::clone::<Option<String>>(args.prefix),
            separator: &crate::clone::<Option<String>>(args.separator),
        });


        let result = RandomPetResult {
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            length: crate::random_to_domain_mapper::<Option<i32>>(result.length),
            prefix: crate::random_to_domain_mapper::<Option<String>>(result.prefix),
            separator: crate::random_to_domain_mapper::<Option<String>>(result.separator),
        };

        result
    }

}


mod random_shuffle {

    pub struct RandomShuffleArgs<'a> {
        pub name: &'a str,
        pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub resultCount: pulumi_wasm_rust::Output<Option<i32>>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RandomShuffleResult {
        pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub resultCount: pulumi_wasm_rust::Output<Option<i32>>,
        pub results: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }

    fn random_shuffle<'a>(args: RandomShuffleArgs<'a>) -> RandomShuffleResult {

        let result = crate::bindings::pulumi::random::random_shuffle::invoke(args.name, &crate::bindings::pulumi::random::random_shuffle::Args {
            inputs: &crate::clone::<Vec<String>>(args.inputs),
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            result_count: &crate::clone::<Option<i32>>(args.resultCount),
            seed: &crate::clone::<Option<String>>(args.seed),
        });


        let result = RandomShuffleResult {
            inputs: crate::random_to_domain_mapper::<Vec<String>>(result.inputs),
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            resultCount: crate::random_to_domain_mapper::<Option<i32>>(result.result_count),
            results: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.results),
            seed: crate::random_to_domain_mapper::<Option<String>>(result.seed),
        };

        result
    }

}


mod random_string {

    pub struct RandomStringArgs<'a> {
        pub name: &'a str,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        pub minLower: pulumi_wasm_rust::Output<Option<i32>>,
        pub minNumeric: pulumi_wasm_rust::Output<Option<i32>>,
        pub minSpecial: pulumi_wasm_rust::Output<Option<i32>>,
        pub minUpper: pulumi_wasm_rust::Output<Option<i32>>,
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
        pub overrideSpecial: pulumi_wasm_rust::Output<Option<String>>,
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub struct RandomStringResult {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        pub minLower: pulumi_wasm_rust::Output<Option<i32>>,
        pub minNumeric: pulumi_wasm_rust::Output<Option<i32>>,
        pub minSpecial: pulumi_wasm_rust::Output<Option<i32>>,
        pub minUpper: pulumi_wasm_rust::Output<Option<i32>>,
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
        pub overrideSpecial: pulumi_wasm_rust::Output<Option<String>>,
        pub result: pulumi_wasm_rust::Output<Option<String>>,
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }

    fn random_string<'a>(args: RandomStringArgs<'a>) -> RandomStringResult {

        let result = crate::bindings::pulumi::random::random_string::invoke(args.name, &crate::bindings::pulumi::random::random_string::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
            length: &crate::clone::<i32>(args.length),
            lower: &crate::clone::<Option<bool>>(args.lower),
            min_lower: &crate::clone::<Option<i32>>(args.minLower),
            min_numeric: &crate::clone::<Option<i32>>(args.minNumeric),
            min_special: &crate::clone::<Option<i32>>(args.minSpecial),
            min_upper: &crate::clone::<Option<i32>>(args.minUpper),
            number: &crate::clone::<Option<bool>>(args.number),
            numeric: &crate::clone::<Option<bool>>(args.numeric),
            override_special: &crate::clone::<Option<String>>(args.overrideSpecial),
            special: &crate::clone::<Option<bool>>(args.special),
            upper: &crate::clone::<Option<bool>>(args.upper),
        });


        let result = RandomStringResult {
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            length: crate::random_to_domain_mapper::<i32>(result.length),
            lower: crate::random_to_domain_mapper::<Option<bool>>(result.lower),
            minLower: crate::random_to_domain_mapper::<Option<i32>>(result.min_lower),
            minNumeric: crate::random_to_domain_mapper::<Option<i32>>(result.min_numeric),
            minSpecial: crate::random_to_domain_mapper::<Option<i32>>(result.min_special),
            minUpper: crate::random_to_domain_mapper::<Option<i32>>(result.min_upper),
            number: crate::random_to_domain_mapper::<Option<bool>>(result.number),
            numeric: crate::random_to_domain_mapper::<Option<bool>>(result.numeric),
            overrideSpecial: crate::random_to_domain_mapper::<Option<String>>(result.override_special),
            result: crate::random_to_domain_mapper::<Option<String>>(result.result),
            special: crate::random_to_domain_mapper::<Option<bool>>(result.special),
            upper: crate::random_to_domain_mapper::<Option<bool>>(result.upper),
        };

        result
    }

}


mod random_uuid {

    pub struct RandomUuidArgs<'a> {
        pub name: &'a str,
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub struct RandomUuidResult {
        pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub result: pulumi_wasm_rust::Output<Option<String>>,
    }

    fn random_uuid<'a>(args: RandomUuidArgs<'a>) -> RandomUuidResult {

        let result = crate::bindings::pulumi::random::random_uuid::invoke(args.name, &crate::bindings::pulumi::random::random_uuid::Args {
            keepers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.keepers),
        });


        let result = RandomUuidResult {
            keepers: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.keepers),
            result: crate::random_to_domain_mapper::<Option<String>>(result.result),
        };

        result
    }

}

