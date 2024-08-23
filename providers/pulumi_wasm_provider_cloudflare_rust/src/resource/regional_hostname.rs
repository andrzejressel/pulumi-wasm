pub struct RegionalHostnameArgs {
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub region_key: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RegionalHostnameResult {
    pub created_on: pulumi_wasm_rust::Output<String>,
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub region_key: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: RegionalHostnameArgs) -> RegionalHostnameResult {
    let result = crate::bindings::pulumi::cloudflare::regional_hostname::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::regional_hostname::Args {
            hostname: args.hostname.get_inner(),
            region_key: args.region_key.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    RegionalHostnameResult {
        created_on: crate::into_domain(result.created_on),
        hostname: crate::into_domain(result.hostname),
        region_key: crate::into_domain(result.region_key),
        zone_id: crate::into_domain(result.zone_id),
    }
}
