pub struct ZoneHoldArgs {
    pub hold: pulumi_wasm_rust::Output<bool>,
    pub hold_after: pulumi_wasm_rust::Output<Option<String>>,
    pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneHoldResult {
    pub hold: pulumi_wasm_rust::Output<bool>,
    pub hold_after: pulumi_wasm_rust::Output<String>,
    pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ZoneHoldArgs) -> ZoneHoldResult {
    let result = crate::bindings::pulumi::cloudflare::zone_hold::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_hold::Args {
            hold: args.hold.get_inner(),
            hold_after: args.hold_after.get_inner(),
            include_subdomains: args.include_subdomains.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ZoneHoldResult {
        hold: crate::into_domain(result.hold),
        hold_after: crate::into_domain(result.hold_after),
        include_subdomains: crate::into_domain(result.include_subdomains),
        zone_id: crate::into_domain(result.zone_id),
    }
}
