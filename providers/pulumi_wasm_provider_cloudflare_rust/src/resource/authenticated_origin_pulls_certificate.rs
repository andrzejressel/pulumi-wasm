pub struct AuthenticatedOriginPullsCertificateArgs {
    pub certificate: pulumi_wasm_rust::Output<String>,
    pub private_key: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct AuthenticatedOriginPullsCertificateResult {
    pub certificate: pulumi_wasm_rust::Output<String>,
    pub expires_on: pulumi_wasm_rust::Output<String>,
    pub issuer: pulumi_wasm_rust::Output<String>,
    pub private_key: pulumi_wasm_rust::Output<String>,
    pub serial_number: pulumi_wasm_rust::Output<String>,
    pub signature: pulumi_wasm_rust::Output<String>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(
    name: &str,
    args: AuthenticatedOriginPullsCertificateArgs,
) -> AuthenticatedOriginPullsCertificateResult {
    let result =
        crate::bindings::pulumi::cloudflare::authenticated_origin_pulls_certificate::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::authenticated_origin_pulls_certificate::Args {
                certificate: args.certificate.get_inner(),
                private_key: args.private_key.get_inner(),
                type_: args.type_.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

    AuthenticatedOriginPullsCertificateResult {
        certificate: crate::into_domain(result.certificate),
        expires_on: crate::into_domain(result.expires_on),
        issuer: crate::into_domain(result.issuer),
        private_key: crate::into_domain(result.private_key),
        serial_number: crate::into_domain(result.serial_number),
        signature: crate::into_domain(result.signature),
        status: crate::into_domain(result.status),
        type_: crate::into_domain(result.type_),
        uploaded_on: crate::into_domain(result.uploaded_on),
        zone_id: crate::into_domain(result.zone_id),
    }
}
