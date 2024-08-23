pub struct RegionalTieredCacheArgs {
    pub value: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RegionalTieredCacheResult {
    pub value: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: RegionalTieredCacheArgs) -> RegionalTieredCacheResult {
    let result = crate::bindings::pulumi::cloudflare::regional_tiered_cache::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::regional_tiered_cache::Args {
            value: args.value.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    RegionalTieredCacheResult {
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
