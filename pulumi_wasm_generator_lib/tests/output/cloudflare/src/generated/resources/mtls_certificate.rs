#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct MtlsCertificateArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub ca: pulumi_wasm_rust::Output<bool>,
    /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub certificates: pulumi_wasm_rust::Output<String>,
    /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub private_key: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct MtlsCertificateResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
    pub ca: pulumi_wasm_rust::Output<bool>,
    /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
    pub certificates: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub issuer: pulumi_wasm_rust::Output<String>,
    /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
    pub private_key: pulumi_wasm_rust::Output<Option<String>>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub serial_number: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub signature: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: MtlsCertificateArgs) -> MtlsCertificateResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let ca_binding = args.ca.get_inner();
    let certificates_binding = args.certificates.get_inner();
    let name_binding = args.name.get_inner();
    let private_key_binding = args.private_key.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/mtlsCertificate:MtlsCertificate".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "ca".into(),
                value: &ca_binding,
            },
            register_interface::ObjectField {
                name: "certificates".into(),
                value: &certificates_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "privateKey".into(),
                value: &private_key_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "ca".into() },
            register_interface::ResultField { name : "certificates".into() },
            register_interface::ResultField { name : "expiresOn".into() },
            register_interface::ResultField { name : "issuer".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "privateKey".into() },
            register_interface::ResultField { name : "serialNumber".into() },
            register_interface::ResultField { name : "signature".into() },
            register_interface::ResultField { name : "uploadedOn".into() },
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
    MtlsCertificateResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        ca: into_domain(hashmap.remove("ca").unwrap()),
        certificates: into_domain(hashmap.remove("certificates").unwrap()),
        expires_on: into_domain(hashmap.remove("expiresOn").unwrap()),
        issuer: into_domain(hashmap.remove("issuer").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        private_key: into_domain(hashmap.remove("privateKey").unwrap()),
        serial_number: into_domain(hashmap.remove("serialNumber").unwrap()),
        signature: into_domain(hashmap.remove("signature").unwrap()),
        uploaded_on: into_domain(hashmap.remove("uploadedOn").unwrap()),
    }
}
