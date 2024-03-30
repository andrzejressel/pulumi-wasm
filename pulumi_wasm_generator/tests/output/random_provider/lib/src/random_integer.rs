pub struct RandomIntegerArgs<'a> {
    pub name: &'a str,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub max: pulumi_wasm_rust::Output<i32>,
    pub min: pulumi_wasm_rust::Output<i32>,
    pub seed: pulumi_wasm_rust::Output<String>,
}

pub struct RandomIntegerResult<'a> {
    pub name: &'a str,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub max: pulumi_wasm_rust::Output<i32>,
    pub min: pulumi_wasm_rust::Output<i32>,
    pub result: pulumi_wasm_rust::Output<i32>,
    pub seed: pulumi_wasm_rust::Output<String>,
}

fn random_integer<'a>(args: RandomIntegerArgs<'a>) -> RandomIntegerResult<'a> {

    crate::bindings::pulumi::random::random_integer::invoke(args.name, &crate::bindings::pulumi::random::random_integer::Args {
        keepers: &crate::clone::<std::collections::HashMap<String, String>>(args.keepers),
        max: &crate::clone::<i32>(args.max),
        min: &crate::clone::<i32>(args.min),
        seed: &crate::clone::<String>(args.seed),
    });

    todo!();

}