pub struct TieredCacheArgs {
    pub cache_type: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct TieredCacheResult {
    pub cache_type: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: TieredCacheArgs) -> TieredCacheResult {
    let result = crate::bindings::pulumi::cloudflare::tiered_cache::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::tiered_cache::Args {
            cache_type: args.cache_type.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    TieredCacheResult {
        cache_type: crate::into_domain(result.cache_type),
        zone_id: crate::into_domain(result.zone_id),
    }
}
