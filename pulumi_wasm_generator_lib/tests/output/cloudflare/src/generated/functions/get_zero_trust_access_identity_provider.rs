#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetZeroTrustAccessIdentityProviderArgs {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Access Identity Provider name to search for.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct GetZeroTrustAccessIdentityProviderResult {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Access Identity Provider name to search for.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Access Identity Provider Type.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(
    args: GetZeroTrustAccessIdentityProviderArgs,
) -> GetZeroTrustAccessIdentityProviderResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let name_binding = args.name.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getZeroTrustAccessIdentityProvider:getZeroTrustAccessIdentityProvider"
            .into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
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
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "type".into() },
            register_interface::ResultField { name : "zoneId".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    GetZeroTrustAccessIdentityProviderResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}