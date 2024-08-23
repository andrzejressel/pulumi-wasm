pub struct TunnelArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub config_src: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub secret: pulumi_wasm_rust::Output<String>,
}

pub struct TunnelResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub cname: pulumi_wasm_rust::Output<String>,
    pub config_src: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub secret: pulumi_wasm_rust::Output<String>,
    pub tunnel_token: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: TunnelArgs) -> TunnelResult {
    let result = crate::bindings::pulumi::cloudflare::tunnel::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::tunnel::Args {
            account_id: args.account_id.get_inner(),
            config_src: args.config_src.get_inner(),
            name: args.name.get_inner(),
            secret: args.secret.get_inner(),
        },
    );

    TunnelResult {
        account_id: crate::into_domain(result.account_id),
        cname: crate::into_domain(result.cname),
        config_src: crate::into_domain(result.config_src),
        name: crate::into_domain(result.name),
        secret: crate::into_domain(result.secret),
        tunnel_token: crate::into_domain(result.tunnel_token),
    }
}
