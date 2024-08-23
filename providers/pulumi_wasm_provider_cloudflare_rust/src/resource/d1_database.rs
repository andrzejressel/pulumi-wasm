pub struct D1DatabaseArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct D1DatabaseResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub version: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: D1DatabaseArgs) -> D1DatabaseResult {
    let result = crate::bindings::pulumi::cloudflare::d1_database::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::d1_database::Args {
            account_id: args.account_id.get_inner(),
            name: args.name.get_inner(),
        },
    );

    D1DatabaseResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
        version: crate::into_domain(result.version),
    }
}
