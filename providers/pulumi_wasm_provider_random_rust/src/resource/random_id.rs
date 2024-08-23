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

pub fn create(name: &str, args: RandomIdArgs) -> RandomIdResult {
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
