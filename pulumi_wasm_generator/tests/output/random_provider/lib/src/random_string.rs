pub struct RandomStringArgs<'a> {
    pub name: &'a str,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub length: pulumi_wasm_rust::Output<i32>,
    pub lower: pulumi_wasm_rust::Output<bool>,
    pub minLower: pulumi_wasm_rust::Output<i32>,
    pub minNumeric: pulumi_wasm_rust::Output<i32>,
    pub minSpecial: pulumi_wasm_rust::Output<i32>,
    pub minUpper: pulumi_wasm_rust::Output<i32>,
    pub number: pulumi_wasm_rust::Output<bool>,
    pub numeric: pulumi_wasm_rust::Output<bool>,
    pub overrideSpecial: pulumi_wasm_rust::Output<String>,
    pub special: pulumi_wasm_rust::Output<bool>,
    pub upper: pulumi_wasm_rust::Output<bool>,
}

pub struct RandomStringResult<'a> {
    pub name: &'a str,
    pub keepers: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub length: pulumi_wasm_rust::Output<i32>,
    pub lower: pulumi_wasm_rust::Output<bool>,
    pub minLower: pulumi_wasm_rust::Output<i32>,
    pub minNumeric: pulumi_wasm_rust::Output<i32>,
    pub minSpecial: pulumi_wasm_rust::Output<i32>,
    pub minUpper: pulumi_wasm_rust::Output<i32>,
    pub number: pulumi_wasm_rust::Output<bool>,
    pub numeric: pulumi_wasm_rust::Output<bool>,
    pub overrideSpecial: pulumi_wasm_rust::Output<String>,
    pub result: pulumi_wasm_rust::Output<String>,
    pub special: pulumi_wasm_rust::Output<bool>,
    pub upper: pulumi_wasm_rust::Output<bool>,
}

fn random_string<'a>(args: RandomStringArgs<'a>) -> RandomStringResult<'a> {

    crate::bindings::pulumi::random::random_string::invoke(args.name, &crate::bindings::pulumi::random::random_string::Args {
        keepers: &crate::clone::<std::collections::HashMap<String, String>>(args.keepers),
        length: &crate::clone::<i32>(args.length),
        lower: &crate::clone::<bool>(args.lower),
        min_lower: &crate::clone::<i32>(args.minLower),
        min_numeric: &crate::clone::<i32>(args.minNumeric),
        min_special: &crate::clone::<i32>(args.minSpecial),
        min_upper: &crate::clone::<i32>(args.minUpper),
        number: &crate::clone::<bool>(args.number),
        numeric: &crate::clone::<bool>(args.numeric),
        override_special: &crate::clone::<String>(args.overrideSpecial),
        special: &crate::clone::<bool>(args.special),
        upper: &crate::clone::<bool>(args.upper),
    });

    todo!();

}