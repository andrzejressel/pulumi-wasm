pub struct RandomBytesArgs<'a> {
    pub name: &'a str,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub length: pulumi_wasm_rust::Output<i32>,
}

pub struct RandomBytesResult<'a> {
    pub name: &'a str,
    pub base64: pulumi_wasm_rust::Output<String>,
    pub hex: pulumi_wasm_rust::Output<String>,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub length: pulumi_wasm_rust::Output<i32>,
}

fn random_bytes<'a>(args: RandomBytesArgs<'a>) -> RandomBytesResult<'a> {

    crate::bindings::pulumi::random::random_bytes::invoke(args.name, &crate::bindings::pulumi::random::random_bytes::Args {
        keepers: &crate::clone::<std::collections::HashMap<String, String>>(args.keepers),
        length: &crate::clone::<i32>(args.length),
    });

    todo!();

}