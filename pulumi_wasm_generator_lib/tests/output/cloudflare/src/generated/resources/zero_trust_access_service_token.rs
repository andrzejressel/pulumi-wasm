#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessServiceTokenArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
    #[builder(into, default)]
    pub duration: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default)]
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// Friendly name of the token's intent.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct ZeroTrustAccessServiceTokenResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Client ID associated with the Service Token. **Modifying this attribute will force creation of a new resource.**
    pub client_id: pulumi_wasm_rust::Output<String>,
    /// A secret for interacting with Access protocols. **Modifying this attribute will force creation of a new resource.**
    pub client_secret: pulumi_wasm_rust::Output<String>,
    /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
    pub duration: pulumi_wasm_rust::Output<String>,
    /// Date when the token expires.
    pub expires_at: pulumi_wasm_rust::Output<String>,
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// Friendly name of the token's intent.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: ZeroTrustAccessServiceTokenArgs,
) -> ZeroTrustAccessServiceTokenResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let duration_binding = args.duration.get_inner();
    let min_days_for_renewal_binding = args.min_days_for_renewal.get_inner();
    let name_binding = args.name.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zeroTrustAccessServiceToken:ZeroTrustAccessServiceToken"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "duration".into(),
                value: &duration_binding,
            },
            register_interface::ObjectField {
                name: "minDaysForRenewal".into(),
                value: &min_days_for_renewal_binding,
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
            register_interface::ResultField { name : "clientId".into() },
            register_interface::ResultField { name : "clientSecret".into() },
            register_interface::ResultField { name : "duration".into() },
            register_interface::ResultField { name : "expiresAt".into() },
            register_interface::ResultField { name : "minDaysForRenewal".into() },
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
    ZeroTrustAccessServiceTokenResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        client_id: into_domain(hashmap.remove("clientId").unwrap()),
        client_secret: into_domain(hashmap.remove("clientSecret").unwrap()),
        duration: into_domain(hashmap.remove("duration").unwrap()),
        expires_at: into_domain(hashmap.remove("expiresAt").unwrap()),
        min_days_for_renewal: into_domain(hashmap.remove("minDaysForRenewal").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}