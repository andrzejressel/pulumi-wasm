pub struct OriginCaCertificateArgs {
    pub csr: pulumi_wasm_rust::Output<String>,
    pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    pub request_type: pulumi_wasm_rust::Output<String>,
    pub requested_validity: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct OriginCaCertificateResult {
    pub certificate: pulumi_wasm_rust::Output<String>,
    pub csr: pulumi_wasm_rust::Output<String>,
    pub expires_on: pulumi_wasm_rust::Output<String>,
    pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    pub request_type: pulumi_wasm_rust::Output<String>,
    pub requested_validity: pulumi_wasm_rust::Output<i32>,
}

pub fn create(name: &str, args: OriginCaCertificateArgs) -> OriginCaCertificateResult {
    let result = crate::bindings::pulumi::cloudflare::origin_ca_certificate::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::origin_ca_certificate::Args {
            csr: args.csr.get_inner(),
            hostnames: args.hostnames.get_inner(),
            min_days_for_renewal: args.min_days_for_renewal.get_inner(),
            request_type: args.request_type.get_inner(),
            requested_validity: args.requested_validity.get_inner(),
        },
    );

    OriginCaCertificateResult {
        certificate: crate::into_domain(result.certificate),
        csr: crate::into_domain(result.csr),
        expires_on: crate::into_domain(result.expires_on),
        hostnames: crate::into_domain(result.hostnames),
        min_days_for_renewal: crate::into_domain(result.min_days_for_renewal),
        request_type: crate::into_domain(result.request_type),
        requested_validity: crate::into_domain(result.requested_validity),
    }
}
