pub struct RandomShuffleArgs<'a> {
    pub name: &'a str,
    pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub resultCount: pulumi_wasm_rust::Output<i32>,
    pub seed: pulumi_wasm_rust::Output<String>,
}

pub struct RandomShuffleResult<'a> {
    pub name: &'a str,
    pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub resultCount: pulumi_wasm_rust::Output<i32>,
    pub results: pulumi_wasm_rust::Output<Vec<String>>,
    pub seed: pulumi_wasm_rust::Output<String>,
}

fn random_shuffle<'a>(args: RandomShuffleArgs<'a>) -> RandomShuffleResult<'a> {

    crate::bindings::pulumi::random::random_shuffle::invoke(args.name, &crate::bindings::pulumi::random::random_shuffle::Args {
        inputs: &crate::clone::<Vec<String>>(args.inputs),
        keepers: &crate::clone::<std::collections::HashMap<String, String>>(args.keepers),
        result_count: &crate::clone::<i32>(args.resultCount),
        seed: &crate::clone::<String>(args.seed),
    });

    todo!();

}