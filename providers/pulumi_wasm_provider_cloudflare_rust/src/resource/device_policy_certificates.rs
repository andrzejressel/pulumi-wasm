pub struct DevicePolicyCertificatesArgs {
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct DevicePolicyCertificatesResult {
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: DevicePolicyCertificatesArgs) -> DevicePolicyCertificatesResult {
    let result = crate::bindings::pulumi::cloudflare::device_policy_certificates::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::device_policy_certificates::Args {
            enabled: args.enabled.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    DevicePolicyCertificatesResult {
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
