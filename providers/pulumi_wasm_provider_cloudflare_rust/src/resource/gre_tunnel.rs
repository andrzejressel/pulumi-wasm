pub struct GreTunnelArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
    pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub health_check_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub health_check_target: pulumi_wasm_rust::Output<Option<String>>,
    pub health_check_type: pulumi_wasm_rust::Output<Option<String>>,
    pub interface_address: pulumi_wasm_rust::Output<String>,
    pub mtu: pulumi_wasm_rust::Output<Option<i32>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct GreTunnelResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
    pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub health_check_enabled: pulumi_wasm_rust::Output<bool>,
    pub health_check_target: pulumi_wasm_rust::Output<String>,
    pub health_check_type: pulumi_wasm_rust::Output<String>,
    pub interface_address: pulumi_wasm_rust::Output<String>,
    pub mtu: pulumi_wasm_rust::Output<i32>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub ttl: pulumi_wasm_rust::Output<i32>,
}

pub fn create(name: &str, args: GreTunnelArgs) -> GreTunnelResult {
    let result = crate::bindings::pulumi::cloudflare::gre_tunnel::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::gre_tunnel::Args {
            account_id: args.account_id.get_inner(),
            cloudflare_gre_endpoint: args.cloudflare_gre_endpoint.get_inner(),
            customer_gre_endpoint: args.customer_gre_endpoint.get_inner(),
            description: args.description.get_inner(),
            health_check_enabled: args.health_check_enabled.get_inner(),
            health_check_target: args.health_check_target.get_inner(),
            health_check_type: args.health_check_type.get_inner(),
            interface_address: args.interface_address.get_inner(),
            mtu: args.mtu.get_inner(),
            name: args.name.get_inner(),
            ttl: args.ttl.get_inner(),
        },
    );

    GreTunnelResult {
        account_id: crate::into_domain(result.account_id),
        cloudflare_gre_endpoint: crate::into_domain(result.cloudflare_gre_endpoint),
        customer_gre_endpoint: crate::into_domain(result.customer_gre_endpoint),
        description: crate::into_domain(result.description),
        health_check_enabled: crate::into_domain(result.health_check_enabled),
        health_check_target: crate::into_domain(result.health_check_target),
        health_check_type: crate::into_domain(result.health_check_type),
        interface_address: crate::into_domain(result.interface_address),
        mtu: crate::into_domain(result.mtu),
        name: crate::into_domain(result.name),
        ttl: crate::into_domain(result.ttl),
    }
}
