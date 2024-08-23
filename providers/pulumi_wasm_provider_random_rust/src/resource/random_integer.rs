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

pub fn create(name: &str, args: RandomIntegerArgs) -> RandomIntegerResult {
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
