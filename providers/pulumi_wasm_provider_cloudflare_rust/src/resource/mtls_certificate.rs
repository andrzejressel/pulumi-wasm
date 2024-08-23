pub struct MtlsCertificateArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub ca: pulumi_wasm_rust::Output<bool>,
    pub certificates: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    pub private_key: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct MtlsCertificateResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub ca: pulumi_wasm_rust::Output<bool>,
    pub certificates: pulumi_wasm_rust::Output<String>,
    pub expires_on: pulumi_wasm_rust::Output<String>,
    pub issuer: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    pub private_key: pulumi_wasm_rust::Output<Option<String>>,
    pub serial_number: pulumi_wasm_rust::Output<String>,
    pub signature: pulumi_wasm_rust::Output<String>,
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: MtlsCertificateArgs) -> MtlsCertificateResult {
    let result = crate::bindings::pulumi::cloudflare::mtls_certificate::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::mtls_certificate::Args {
            account_id: args.account_id.get_inner(),
            ca: args.ca.get_inner(),
            certificates: args.certificates.get_inner(),
            name: args.name.get_inner(),
            private_key: args.private_key.get_inner(),
        },
    );

    MtlsCertificateResult {
        account_id: crate::into_domain(result.account_id),
        ca: crate::into_domain(result.ca),
        certificates: crate::into_domain(result.certificates),
        expires_on: crate::into_domain(result.expires_on),
        issuer: crate::into_domain(result.issuer),
        name: crate::into_domain(result.name),
        private_key: crate::into_domain(result.private_key),
        serial_number: crate::into_domain(result.serial_number),
        signature: crate::into_domain(result.signature),
        uploaded_on: crate::into_domain(result.uploaded_on),
    }
}
