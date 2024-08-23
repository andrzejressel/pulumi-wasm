pub struct TunnelRouteArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    pub network: pulumi_wasm_rust::Output<String>,
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
    pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct TunnelRouteResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    pub network: pulumi_wasm_rust::Output<String>,
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
    pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: TunnelRouteArgs) -> TunnelRouteResult {
    let result = crate::bindings::pulumi::cloudflare::tunnel_route::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::tunnel_route::Args {
            account_id: args.account_id.get_inner(),
            comment: args.comment.get_inner(),
            network: args.network.get_inner(),
            tunnel_id: args.tunnel_id.get_inner(),
            virtual_network_id: args.virtual_network_id.get_inner(),
        },
    );

    TunnelRouteResult {
        account_id: crate::into_domain(result.account_id),
        comment: crate::into_domain(result.comment),
        network: crate::into_domain(result.network),
        tunnel_id: crate::into_domain(result.tunnel_id),
        virtual_network_id: crate::into_domain(result.virtual_network_id),
    }
}
