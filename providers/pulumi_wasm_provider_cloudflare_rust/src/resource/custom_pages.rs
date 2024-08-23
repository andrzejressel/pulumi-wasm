pub struct CustomPagesArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub state: pulumi_wasm_rust::Output<Option<String>>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub url: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct CustomPagesResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub state: pulumi_wasm_rust::Output<Option<String>>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub url: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: CustomPagesArgs) -> CustomPagesResult {
    let result = crate::bindings::pulumi::cloudflare::custom_pages::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::custom_pages::Args {
            account_id: args.account_id.get_inner(),
            state: args.state.get_inner(),
            type_: args.type_.get_inner(),
            url: args.url.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    CustomPagesResult {
        account_id: crate::into_domain(result.account_id),
        state: crate::into_domain(result.state),
        type_: crate::into_domain(result.type_),
        url: crate::into_domain(result.url),
        zone_id: crate::into_domain(result.zone_id),
    }
}
