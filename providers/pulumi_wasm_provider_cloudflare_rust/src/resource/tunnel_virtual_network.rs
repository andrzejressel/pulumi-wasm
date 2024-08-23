pub struct TunnelVirtualNetworkArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct TunnelVirtualNetworkResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: TunnelVirtualNetworkArgs) -> TunnelVirtualNetworkResult {
    let result = crate::bindings::pulumi::cloudflare::tunnel_virtual_network::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::tunnel_virtual_network::Args {
            account_id: args.account_id.get_inner(),
            comment: args.comment.get_inner(),
            is_default_network: args.is_default_network.get_inner(),
            name: args.name.get_inner(),
        },
    );

    TunnelVirtualNetworkResult {
        account_id: crate::into_domain(result.account_id),
        comment: crate::into_domain(result.comment),
        is_default_network: crate::into_domain(result.is_default_network),
        name: crate::into_domain(result.name),
    }
}
