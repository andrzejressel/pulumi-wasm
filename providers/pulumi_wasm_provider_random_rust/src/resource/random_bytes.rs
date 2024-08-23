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

pub fn create(name: &str, args: RandomBytesArgs) -> RandomBytesResult {
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
