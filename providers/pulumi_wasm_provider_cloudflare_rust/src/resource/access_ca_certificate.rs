pub struct AccessCaCertificateArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub application_id: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessCaCertificateResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub application_id: pulumi_wasm_rust::Output<String>,
    pub aud: pulumi_wasm_rust::Output<String>,
    pub public_key: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: AccessCaCertificateArgs) -> AccessCaCertificateResult {
    let result = crate::bindings::pulumi::cloudflare::access_ca_certificate::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_ca_certificate::Args {
            account_id: args.account_id.get_inner(),
            application_id: args.application_id.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessCaCertificateResult {
        account_id: crate::into_domain(result.account_id),
        application_id: crate::into_domain(result.application_id),
        aud: crate::into_domain(result.aud),
        public_key: crate::into_domain(result.public_key),
        zone_id: crate::into_domain(result.zone_id),
    }
}
