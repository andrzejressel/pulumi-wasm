pub struct ZoneCacheReserveArgs {
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneCacheReserveResult {
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ZoneCacheReserveArgs) -> ZoneCacheReserveResult {
    let result = crate::bindings::pulumi::cloudflare::zone_cache_reserve::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_cache_reserve::Args {
            enabled: args.enabled.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ZoneCacheReserveResult {
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
