pub struct TeamsProxyEndpointArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub ips: pulumi_wasm_rust::Output<Vec<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct TeamsProxyEndpointResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub ips: pulumi_wasm_rust::Output<Vec<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub subdomain: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: TeamsProxyEndpointArgs) -> TeamsProxyEndpointResult {
    let result = crate::bindings::pulumi::cloudflare::teams_proxy_endpoint::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::teams_proxy_endpoint::Args {
            account_id: args.account_id.get_inner(),
            ips: args.ips.get_inner(),
            name: args.name.get_inner(),
        },
    );

    TeamsProxyEndpointResult {
        account_id: crate::into_domain(result.account_id),
        ips: crate::into_domain(result.ips),
        name: crate::into_domain(result.name),
        subdomain: crate::into_domain(result.subdomain),
    }
}
