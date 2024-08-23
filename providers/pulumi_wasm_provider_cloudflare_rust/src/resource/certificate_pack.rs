pub struct CertificatePackArgs {
    pub certificate_authority: pulumi_wasm_rust::Output<String>,
    pub cloudflare_branding: pulumi_wasm_rust::Output<Option<bool>>,
    pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub validation_errors:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::CertificatePackValidationError>>>,
    pub validation_method: pulumi_wasm_rust::Output<String>,
    pub validation_records:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::CertificatePackValidationRecord>>>,
    pub validity_days: pulumi_wasm_rust::Output<i32>,
    pub wait_for_active_status: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CertificatePackResult {
    pub certificate_authority: pulumi_wasm_rust::Output<String>,
    pub cloudflare_branding: pulumi_wasm_rust::Output<Option<bool>>,
    pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub validation_errors:
        pulumi_wasm_rust::Output<Vec<crate::types::CertificatePackValidationError>>,
    pub validation_method: pulumi_wasm_rust::Output<String>,
    pub validation_records:
        pulumi_wasm_rust::Output<Vec<crate::types::CertificatePackValidationRecord>>,
    pub validity_days: pulumi_wasm_rust::Output<i32>,
    pub wait_for_active_status: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: CertificatePackArgs) -> CertificatePackResult {
    let result = crate::bindings::pulumi::cloudflare::certificate_pack::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::certificate_pack::Args {
            certificate_authority: args.certificate_authority.get_inner(),
            cloudflare_branding: args.cloudflare_branding.get_inner(),
            hosts: args.hosts.get_inner(),
            type_: args.type_.get_inner(),
            validation_errors: args.validation_errors.get_inner(),
            validation_method: args.validation_method.get_inner(),
            validation_records: args.validation_records.get_inner(),
            validity_days: args.validity_days.get_inner(),
            wait_for_active_status: args.wait_for_active_status.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    CertificatePackResult {
        certificate_authority: crate::into_domain(result.certificate_authority),
        cloudflare_branding: crate::into_domain(result.cloudflare_branding),
        hosts: crate::into_domain(result.hosts),
        type_: crate::into_domain(result.type_),
        validation_errors: crate::into_domain(result.validation_errors),
        validation_method: crate::into_domain(result.validation_method),
        validation_records: crate::into_domain(result.validation_records),
        validity_days: crate::into_domain(result.validity_days),
        wait_for_active_status: crate::into_domain(result.wait_for_active_status),
        zone_id: crate::into_domain(result.zone_id),
    }
}
