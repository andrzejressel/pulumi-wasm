pub struct AccessMutualTlsCertificateArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub certificate: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessMutualTlsCertificateResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub certificate: pulumi_wasm_rust::Output<Option<String>>,
    pub fingerprint: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(
    name: &str,
    args: AccessMutualTlsCertificateArgs,
) -> AccessMutualTlsCertificateResult {
    let result = crate::bindings::pulumi::cloudflare::access_mutual_tls_certificate::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_mutual_tls_certificate::Args {
            account_id: args.account_id.get_inner(),
            associated_hostnames: args.associated_hostnames.get_inner(),
            certificate: args.certificate.get_inner(),
            name: args.name.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessMutualTlsCertificateResult {
        account_id: crate::into_domain(result.account_id),
        associated_hostnames: crate::into_domain(result.associated_hostnames),
        certificate: crate::into_domain(result.certificate),
        fingerprint: crate::into_domain(result.fingerprint),
        name: crate::into_domain(result.name),
        zone_id: crate::into_domain(result.zone_id),
    }
}
