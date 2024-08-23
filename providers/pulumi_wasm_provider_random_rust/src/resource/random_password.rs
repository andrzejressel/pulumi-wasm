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

pub fn create(name: &str, args: RandomPasswordArgs) -> RandomPasswordResult {
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
