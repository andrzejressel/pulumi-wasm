pub struct CustomHostnameFallbackOriginArgs {
    pub origin: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CustomHostnameFallbackOriginResult {
    pub origin: pulumi_wasm_rust::Output<String>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(
    name: &str,
    args: CustomHostnameFallbackOriginArgs,
) -> CustomHostnameFallbackOriginResult {
    let result = crate::bindings::pulumi::cloudflare::custom_hostname_fallback_origin::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::custom_hostname_fallback_origin::Args {
            origin: args.origin.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    CustomHostnameFallbackOriginResult {
        origin: crate::into_domain(result.origin),
        status: crate::into_domain(result.status),
        zone_id: crate::into_domain(result.zone_id),
    }
}
