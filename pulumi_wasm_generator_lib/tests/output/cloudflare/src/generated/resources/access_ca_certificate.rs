#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessCaCertificateArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The Access Application ID to associate with the CA certificate.
    #[builder(into)]
    pub application_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct AccessCaCertificateResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The Access Application ID to associate with the CA certificate.
    pub application_id: pulumi_wasm_rust::Output<String>,
    /// Application Audience (AUD) Tag of the CA certificate.
    pub aud: pulumi_wasm_rust::Output<String>,
    /// Cryptographic public key of the generated CA certificate.
    pub public_key: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessCaCertificateArgs) -> AccessCaCertificateResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let application_id_binding = args.application_id.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/accessCaCertificate:AccessCaCertificate".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "applicationId".into(),
                value: &application_id_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "applicationId".into() },
            register_interface::ResultField { name : "aud".into() },
            register_interface::ResultField { name : "publicKey".into() },
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
    AccessCaCertificateResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        application_id: into_domain(hashmap.remove("applicationId").unwrap()),
        aud: into_domain(hashmap.remove("aud").unwrap()),
        public_key: into_domain(hashmap.remove("publicKey").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
