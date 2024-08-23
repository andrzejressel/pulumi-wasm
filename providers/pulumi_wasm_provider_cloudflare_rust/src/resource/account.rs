pub struct AccountArgs {
    pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccountResult {
    pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: AccountArgs) -> AccountResult {
    let result = crate::bindings::pulumi::cloudflare::account::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::account::Args {
            enforce_twofactor: args.enforce_twofactor.get_inner(),
            name: args.name.get_inner(),
            type_: args.type_.get_inner(),
        },
    );

    AccountResult {
        enforce_twofactor: crate::into_domain(result.enforce_twofactor),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
