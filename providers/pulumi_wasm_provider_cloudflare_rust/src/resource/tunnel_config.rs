pub struct TunnelConfigArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub config: pulumi_wasm_rust::Output<crate::types::TunnelConfigConfig>,
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
}

pub struct TunnelConfigResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub config: pulumi_wasm_rust::Output<crate::types::TunnelConfigConfig>,
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: TunnelConfigArgs) -> TunnelConfigResult {
    let result = crate::bindings::pulumi::cloudflare::tunnel_config::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::tunnel_config::Args {
            account_id: args.account_id.get_inner(),
            config: args.config.get_inner(),
            tunnel_id: args.tunnel_id.get_inner(),
        },
    );

    TunnelConfigResult {
        account_id: crate::into_domain(result.account_id),
        config: crate::into_domain(result.config),
        tunnel_id: crate::into_domain(result.tunnel_id),
    }
}
