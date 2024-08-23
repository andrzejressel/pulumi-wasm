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

pub fn create(name: &str, args: RandomShuffleArgs) -> RandomShuffleResult {
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
