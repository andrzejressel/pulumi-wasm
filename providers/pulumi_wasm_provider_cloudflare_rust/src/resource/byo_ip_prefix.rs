pub struct ByoIpPrefixArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub advertisement: pulumi_wasm_rust::Output<Option<String>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub prefix_id: pulumi_wasm_rust::Output<String>,
}

pub struct ByoIpPrefixResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub advertisement: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<String>,
    pub prefix_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ByoIpPrefixArgs) -> ByoIpPrefixResult {
    let result = crate::bindings::pulumi::cloudflare::byo_ip_prefix::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::byo_ip_prefix::Args {
            account_id: args.account_id.get_inner(),
            advertisement: args.advertisement.get_inner(),
            description: args.description.get_inner(),
            prefix_id: args.prefix_id.get_inner(),
        },
    );

    ByoIpPrefixResult {
        account_id: crate::into_domain(result.account_id),
        advertisement: crate::into_domain(result.advertisement),
        description: crate::into_domain(result.description),
        prefix_id: crate::into_domain(result.prefix_id),
    }
}
