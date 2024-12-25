#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessMtlsCertificateArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The hostnames that will be prompted for this certificate.
    #[builder(into, default)]
    pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The Root CA for your certificates.
    #[builder(into, default)]
    pub certificate: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the certificate.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct ZeroTrustAccessMtlsCertificateResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The hostnames that will be prompted for this certificate.
    pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The Root CA for your certificates.
    pub certificate: pulumi_wasm_rust::Output<Option<String>>,
    pub fingerprint: pulumi_wasm_rust::Output<String>,
    /// The name of the certificate.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: ZeroTrustAccessMtlsCertificateArgs,
) -> ZeroTrustAccessMtlsCertificateResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let associated_hostnames_binding = args.associated_hostnames.get_inner();
    let certificate_binding = args.certificate.get_inner();
    let name_binding = args.name.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zeroTrustAccessMtlsCertificate:ZeroTrustAccessMtlsCertificate"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "associatedHostnames".into(),
                value: &associated_hostnames_binding,
            },
            register_interface::ObjectField {
                name: "certificate".into(),
                value: &certificate_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "associatedHostnames".into() },
            register_interface::ResultField { name : "certificate".into() },
            register_interface::ResultField { name : "fingerprint".into() },
            register_interface::ResultField { name : "name".into() },
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
    ZeroTrustAccessMtlsCertificateResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        associated_hostnames: into_domain(
            hashmap.remove("associatedHostnames").unwrap(),
        ),
        certificate: into_domain(hashmap.remove("certificate").unwrap()),
        fingerprint: into_domain(hashmap.remove("fingerprint").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
