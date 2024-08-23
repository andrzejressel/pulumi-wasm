pub struct TotalTlsArgs {
    pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct TotalTlsResult {
    pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: TotalTlsArgs) -> TotalTlsResult {
    let result = crate::bindings::pulumi::cloudflare::total_tls::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::total_tls::Args {
            certificate_authority: args.certificate_authority.get_inner(),
            enabled: args.enabled.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    TotalTlsResult {
        certificate_authority: crate::into_domain(result.certificate_authority),
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
