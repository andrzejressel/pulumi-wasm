pub struct RandomUuidArgs<'a> {
    pub name: &'a str,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
}

pub struct RandomUuidResult<'a> {
    pub name: &'a str,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub result: pulumi_wasm_rust::Output<String>,
}

fn random_uuid<'a>(args: RandomUuidArgs<'a>) -> RandomUuidResult<'a> {

    crate::bindings::pulumi::random::random_uuid::invoke(args.name, &crate::bindings::pulumi::random::random_uuid::Args {
        keepers: &crate::clone::<std::collections::HashMap<String, String>>(args.keepers),
    });

    todo!();

}