#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewayCertificateArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not to activate a certificate. A certificate must be activated to use in Gateway certificate settings. Defaults to `false`.
    #[builder(into, default)]
    pub activate: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
    #[builder(into, default)]
    pub custom: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
    #[builder(into, default)]
    pub gateway_managed: pulumi_wasm_rust::Output<Option<bool>>,
    /// Number of days the generated certificate will be valid, minimum 1 day and maximum 30 years. Defaults to 5 years. Defaults to `1826`. Required when using `gateway_managed`. Conflicts with `custom`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub validity_period_days: pulumi_wasm_rust::Output<Option<i32>>,
}
pub struct ZeroTrustGatewayCertificateResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not to activate a certificate. A certificate must be activated to use in Gateway certificate settings. Defaults to `false`.
    pub activate: pulumi_wasm_rust::Output<Option<bool>>,
    /// The deployment status of the certificate on the edge Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
    pub binding_status: pulumi_wasm_rust::Output<String>,
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
    pub custom: pulumi_wasm_rust::Output<Option<bool>>,
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
    pub gateway_managed: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether the certificate is in use by Gateway for TLS interception and the block page.
    pub in_use: pulumi_wasm_rust::Output<bool>,
    pub qs_pack_id: pulumi_wasm_rust::Output<String>,
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
    /// Number of days the generated certificate will be valid, minimum 1 day and maximum 30 years. Defaults to 5 years. Defaults to `1826`. Required when using `gateway_managed`. Conflicts with `custom`. **Modifying this attribute will force creation of a new resource.**
    pub validity_period_days: pulumi_wasm_rust::Output<Option<i32>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: ZeroTrustGatewayCertificateArgs,
) -> ZeroTrustGatewayCertificateResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let activate_binding = args.activate.get_inner();
    let custom_binding = args.custom.get_inner();
    let gateway_managed_binding = args.gateway_managed.get_inner();
    let validity_period_days_binding = args.validity_period_days.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zeroTrustGatewayCertificate:ZeroTrustGatewayCertificate"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "activate".into(),
                value: &activate_binding,
            },
            register_interface::ObjectField {
                name: "custom".into(),
                value: &custom_binding,
            },
            register_interface::ObjectField {
                name: "gatewayManaged".into(),
                value: &gateway_managed_binding,
            },
            register_interface::ObjectField {
                name: "validityPeriodDays".into(),
                value: &validity_period_days_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "activate".into() },
            register_interface::ResultField { name : "bindingStatus".into() },
            register_interface::ResultField { name : "createdAt".into() },
            register_interface::ResultField { name : "custom".into() },
            register_interface::ResultField { name : "expiresOn".into() },
            register_interface::ResultField { name : "gatewayManaged".into() },
            register_interface::ResultField { name : "inUse".into() },
            register_interface::ResultField { name : "qsPackId".into() },
            register_interface::ResultField { name : "uploadedOn".into() },
            register_interface::ResultField { name : "validityPeriodDays".into() },
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
    ZeroTrustGatewayCertificateResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        activate: into_domain(hashmap.remove("activate").unwrap()),
        binding_status: into_domain(hashmap.remove("bindingStatus").unwrap()),
        created_at: into_domain(hashmap.remove("createdAt").unwrap()),
        custom: into_domain(hashmap.remove("custom").unwrap()),
        expires_on: into_domain(hashmap.remove("expiresOn").unwrap()),
        gateway_managed: into_domain(hashmap.remove("gatewayManaged").unwrap()),
        in_use: into_domain(hashmap.remove("inUse").unwrap()),
        qs_pack_id: into_domain(hashmap.remove("qsPackId").unwrap()),
        uploaded_on: into_domain(hashmap.remove("uploadedOn").unwrap()),
        validity_period_days: into_domain(hashmap.remove("validityPeriodDays").unwrap()),
    }
}
