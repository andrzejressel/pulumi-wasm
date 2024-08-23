pub struct SplitTunnelArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub mode: pulumi_wasm_rust::Output<String>,
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::SplitTunnelTunnel>>,
}

pub struct SplitTunnelResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub mode: pulumi_wasm_rust::Output<String>,
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::SplitTunnelTunnel>>,
}

pub fn create(name: &str, args: SplitTunnelArgs) -> SplitTunnelResult {
    let result = crate::bindings::pulumi::cloudflare::split_tunnel::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::split_tunnel::Args {
            account_id: args.account_id.get_inner(),
            mode: args.mode.get_inner(),
            policy_id: args.policy_id.get_inner(),
            tunnels: args.tunnels.get_inner(),
        },
    );

    SplitTunnelResult {
        account_id: crate::into_domain(result.account_id),
        mode: crate::into_domain(result.mode),
        policy_id: crate::into_domain(result.policy_id),
        tunnels: crate::into_domain(result.tunnels),
    }
}
