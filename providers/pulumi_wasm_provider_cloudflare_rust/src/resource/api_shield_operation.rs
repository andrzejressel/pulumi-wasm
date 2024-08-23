pub struct ApiShieldOperationArgs {
    pub endpoint: pulumi_wasm_rust::Output<String>,
    pub host: pulumi_wasm_rust::Output<String>,
    pub method: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldOperationResult {
    pub endpoint: pulumi_wasm_rust::Output<String>,
    pub host: pulumi_wasm_rust::Output<String>,
    pub method: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ApiShieldOperationArgs) -> ApiShieldOperationResult {
    let result = crate::bindings::pulumi::cloudflare::api_shield_operation::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::api_shield_operation::Args {
            endpoint: args.endpoint.get_inner(),
            host: args.host.get_inner(),
            method: args.method.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ApiShieldOperationResult {
        endpoint: crate::into_domain(result.endpoint),
        host: crate::into_domain(result.host),
        method: crate::into_domain(result.method),
        zone_id: crate::into_domain(result.zone_id),
    }
}
