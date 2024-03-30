pub struct RandomIdArgs<'a> {
    pub name: &'a str,
    pub byteLength: pulumi_wasm_rust::Output<i32>,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub prefix: pulumi_wasm_rust::Output<String>,
}

pub struct RandomIdResult<'a> {
    pub name: &'a str,
    pub b64Std: pulumi_wasm_rust::Output<String>,
    pub b64Url: pulumi_wasm_rust::Output<String>,
    pub byteLength: pulumi_wasm_rust::Output<i32>,
    pub dec: pulumi_wasm_rust::Output<String>,
    pub hex: pulumi_wasm_rust::Output<String>,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub prefix: pulumi_wasm_rust::Output<String>,
}

fn random_id<'a>(args: RandomIdArgs<'a>) -> RandomIdResult<'a> {

    crate::bindings::pulumi::random::random_id::invoke(args.name, &crate::bindings::pulumi::random::random_id::Args {
        byte_length: &crate::clone::<i32>(args.byteLength),
        keepers: &crate::clone::<std::collections::HashMap<String, String>>(args.keepers),
        prefix: &crate::clone::<String>(args.prefix),
    });

    todo!();

}