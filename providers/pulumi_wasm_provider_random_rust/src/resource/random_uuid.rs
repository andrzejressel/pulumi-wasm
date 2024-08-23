pub struct RandomUuidArgs {
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub struct RandomUuidResult {
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    pub result: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: RandomUuidArgs) -> RandomUuidResult {
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
