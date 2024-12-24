#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CertificatePackArgs {
    /// Which certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`, `ssl_com`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub certificate_authority: pulumi_wasm_rust::Output<String>,
    /// Whether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub cloudflare_branding: pulumi_wasm_rust::Output<Option<bool>>,
    /// List of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
    /// Certificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub validation_errors: pulumi_wasm_rust::Output<
        Option<Vec<super::types::CertificatePackValidationError>>,
    >,
    /// Which validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub validation_method: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub validation_records: pulumi_wasm_rust::Output<
        Option<Vec<super::types::CertificatePackValidationRecord>>,
    >,
    /// How long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub validity_days: pulumi_wasm_rust::Output<i32>,
    /// Whether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub wait_for_active_status: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct CertificatePackResult {
    /// Which certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`, `ssl_com`. **Modifying this attribute will force creation of a new resource.**
    pub certificate_authority: pulumi_wasm_rust::Output<String>,
    /// Whether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**
    pub cloudflare_branding: pulumi_wasm_rust::Output<Option<bool>>,
    /// List of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**
    pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
    /// Certificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
    pub validation_errors: pulumi_wasm_rust::Output<
        Vec<super::types::CertificatePackValidationError>,
    >,
    /// Which validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**
    pub validation_method: pulumi_wasm_rust::Output<String>,
    pub validation_records: pulumi_wasm_rust::Output<
        Vec<super::types::CertificatePackValidationRecord>,
    >,
    /// How long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**
    pub validity_days: pulumi_wasm_rust::Output<i32>,
    /// Whether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**
    pub wait_for_active_status: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: CertificatePackArgs) -> CertificatePackResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let certificate_authority_binding = args.certificate_authority.get_inner();
    let cloudflare_branding_binding = args.cloudflare_branding.get_inner();
    let hosts_binding = args.hosts.get_inner();
    let type__binding = args.type_.get_inner();
    let validation_errors_binding = args.validation_errors.get_inner();
    let validation_method_binding = args.validation_method.get_inner();
    let validation_records_binding = args.validation_records.get_inner();
    let validity_days_binding = args.validity_days.get_inner();
    let wait_for_active_status_binding = args.wait_for_active_status.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/certificatePack:CertificatePack".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "certificateAuthority".into(),
                value: &certificate_authority_binding,
            },
            register_interface::ObjectField {
                name: "cloudflareBranding".into(),
                value: &cloudflare_branding_binding,
            },
            register_interface::ObjectField {
                name: "hosts".into(),
                value: &hosts_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
            register_interface::ObjectField {
                name: "validationErrors".into(),
                value: &validation_errors_binding,
            },
            register_interface::ObjectField {
                name: "validationMethod".into(),
                value: &validation_method_binding,
            },
            register_interface::ObjectField {
                name: "validationRecords".into(),
                value: &validation_records_binding,
            },
            register_interface::ObjectField {
                name: "validityDays".into(),
                value: &validity_days_binding,
            },
            register_interface::ObjectField {
                name: "waitForActiveStatus".into(),
                value: &wait_for_active_status_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "certificateAuthority".into() },
            register_interface::ResultField { name : "cloudflareBranding".into() },
            register_interface::ResultField { name : "hosts".into() },
            register_interface::ResultField { name : "type".into() },
            register_interface::ResultField { name : "validationErrors".into() },
            register_interface::ResultField { name : "validationMethod".into() },
            register_interface::ResultField { name : "validationRecords".into() },
            register_interface::ResultField { name : "validityDays".into() },
            register_interface::ResultField { name : "waitForActiveStatus".into() },
            register_interface::ResultField { name : "zoneId".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    CertificatePackResult {
        certificate_authority: into_domain(
            hashmap.remove("certificateAuthority").unwrap(),
        ),
        cloudflare_branding: into_domain(hashmap.remove("cloudflareBranding").unwrap()),
        hosts: into_domain(hashmap.remove("hosts").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        validation_errors: into_domain(hashmap.remove("validationErrors").unwrap()),
        validation_method: into_domain(hashmap.remove("validationMethod").unwrap()),
        validation_records: into_domain(hashmap.remove("validationRecords").unwrap()),
        validity_days: into_domain(hashmap.remove("validityDays").unwrap()),
        wait_for_active_status: into_domain(
            hashmap.remove("waitForActiveStatus").unwrap(),
        ),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
