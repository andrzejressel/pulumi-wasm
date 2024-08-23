pub struct HyperdriveConfigArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub caching: pulumi_wasm_rust::Output<Option<crate::types::HyperdriveConfigCaching>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub origin: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigOrigin>,
}

pub struct HyperdriveConfigResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub caching: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigCaching>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub origin: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigOrigin>,
}

pub fn create(name: &str, args: HyperdriveConfigArgs) -> HyperdriveConfigResult {
    let result = crate::bindings::pulumi::cloudflare::hyperdrive_config::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::hyperdrive_config::Args {
            account_id: args.account_id.get_inner(),
            caching: args.caching.get_inner(),
            name: args.name.get_inner(),
            origin: args.origin.get_inner(),
        },
    );

    HyperdriveConfigResult {
        account_id: crate::into_domain(result.account_id),
        caching: crate::into_domain(result.caching),
        name: crate::into_domain(result.name),
        origin: crate::into_domain(result.origin),
    }
}
