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

pub fn create(name: &str, args: RandomPetArgs) -> RandomPetResult {
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
