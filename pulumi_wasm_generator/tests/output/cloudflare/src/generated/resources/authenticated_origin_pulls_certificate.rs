#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AuthenticatedOriginPullsCertificateArgs {
    /// The public client certificate. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// The private key of the client certificate. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub private_key: pulumi_wasm_rust::Output<String>,
    /// The form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct AuthenticatedOriginPullsCertificateResult {
    /// The public client certificate. **Modifying this attribute will force creation of a new resource.**
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub issuer: pulumi_wasm_rust::Output<String>,
    /// The private key of the client certificate. **Modifying this attribute will force creation of a new resource.**
    pub private_key: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub serial_number: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub signature: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub status: pulumi_wasm_rust::Output<String>,
    /// The form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: AuthenticatedOriginPullsCertificateArgs,
) -> AuthenticatedOriginPullsCertificateResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let certificate_binding = args.certificate.get_inner();
    let private_key_binding = args.private_key.get_inner();
    let type__binding = args.type_.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/authenticatedOriginPullsCertificate:AuthenticatedOriginPullsCertificate"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "certificate".into(),
                value: &certificate_binding,
            },
            register_interface::ObjectField {
                name: "privateKey".into(),
                value: &private_key_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "certificate".into() },
            register_interface::ResultField { name : "expiresOn".into() },
            register_interface::ResultField { name : "issuer".into() },
            register_interface::ResultField { name : "privateKey".into() },
            register_interface::ResultField { name : "serialNumber".into() },
            register_interface::ResultField { name : "signature".into() },
            register_interface::ResultField { name : "status".into() },
            register_interface::ResultField { name : "type".into() },
            register_interface::ResultField { name : "uploadedOn".into() },
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
    AuthenticatedOriginPullsCertificateResult {
        certificate: into_domain(hashmap.remove("certificate").unwrap()),
        expires_on: into_domain(hashmap.remove("expiresOn").unwrap()),
        issuer: into_domain(hashmap.remove("issuer").unwrap()),
        private_key: into_domain(hashmap.remove("privateKey").unwrap()),
        serial_number: into_domain(hashmap.remove("serialNumber").unwrap()),
        signature: into_domain(hashmap.remove("signature").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        uploaded_on: into_domain(hashmap.remove("uploadedOn").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
