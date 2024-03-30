pub struct RandomPetArgs<'a> {
    pub name: &'a str,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub length: pulumi_wasm_rust::Output<i32>,
    pub prefix: pulumi_wasm_rust::Output<String>,
    pub separator: pulumi_wasm_rust::Output<String>,
}

pub struct RandomPetResult<'a> {
    pub name: &'a str,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub length: pulumi_wasm_rust::Output<i32>,
    pub prefix: pulumi_wasm_rust::Output<String>,
    pub separator: pulumi_wasm_rust::Output<String>,
}

fn random_pet<'a>(args: RandomPetArgs<'a>) -> RandomPetResult<'a> {

    crate::bindings::pulumi::random::random_pet::invoke(args.name, &crate::bindings::pulumi::random::random_pet::Args {
        keepers: &crate::clone::<std::collections::HashMap<String, String>>(args.keepers),
        length: &crate::clone::<i32>(args.length),
        prefix: &crate::clone::<String>(args.prefix),
        separator: &crate::clone::<String>(args.separator),
    });

    todo!();

}