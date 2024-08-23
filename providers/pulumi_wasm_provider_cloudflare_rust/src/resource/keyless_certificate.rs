pub struct KeylessCertificateArgs {
    pub bundle_method: pulumi_wasm_rust::Output<Option<String>>,
    pub certificate: pulumi_wasm_rust::Output<String>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub host: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct KeylessCertificateResult {
    pub bundle_method: pulumi_wasm_rust::Output<Option<String>>,
    pub certificate: pulumi_wasm_rust::Output<String>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub host: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: KeylessCertificateArgs) -> KeylessCertificateResult {
    let result = crate::bindings::pulumi::cloudflare::keyless_certificate::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::keyless_certificate::Args {
            bundle_method: args.bundle_method.get_inner(),
            certificate: args.certificate.get_inner(),
            enabled: args.enabled.get_inner(),
            host: args.host.get_inner(),
            name: args.name.get_inner(),
            port: args.port.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    KeylessCertificateResult {
        bundle_method: crate::into_domain(result.bundle_method),
        certificate: crate::into_domain(result.certificate),
        enabled: crate::into_domain(result.enabled),
        host: crate::into_domain(result.host),
        name: crate::into_domain(result.name),
        port: crate::into_domain(result.port),
        status: crate::into_domain(result.status),
        zone_id: crate::into_domain(result.zone_id),
    }
}
