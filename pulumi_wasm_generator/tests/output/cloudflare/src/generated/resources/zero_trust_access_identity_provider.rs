#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessIdentityProviderArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Provider configuration from the [developer documentation](https://developers.cloudflare.com/access/configuring-identity-providers/).
    #[builder(into, default)]
    pub configs: pulumi_wasm_rust::Output<
        Option<Vec<super::types::ZeroTrustAccessIdentityProviderConfig>>,
    >,
    /// Friendly name of the Access Identity Provider configuration.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Configuration for SCIM settings for a given IDP.
    #[builder(into, default)]
    pub scim_configs: pulumi_wasm_rust::Output<
        Option<Vec<super::types::ZeroTrustAccessIdentityProviderScimConfig>>,
    >,
    /// The provider type to use. Available values: `azureAD`, `centrify`, `facebook`, `github`, `google`, `google-apps`, `linkedin`, `oidc`, `okta`, `onelogin`, `onetimepin`, `pingone`, `saml`, `yandex`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct ZeroTrustAccessIdentityProviderResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Provider configuration from the [developer documentation](https://developers.cloudflare.com/access/configuring-identity-providers/).
    pub configs: pulumi_wasm_rust::Output<
        Vec<super::types::ZeroTrustAccessIdentityProviderConfig>,
    >,
    /// Friendly name of the Access Identity Provider configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Configuration for SCIM settings for a given IDP.
    pub scim_configs: pulumi_wasm_rust::Output<
        Vec<super::types::ZeroTrustAccessIdentityProviderScimConfig>,
    >,
    /// The provider type to use. Available values: `azureAD`, `centrify`, `facebook`, `github`, `google`, `google-apps`, `linkedin`, `oidc`, `okta`, `onelogin`, `onetimepin`, `pingone`, `saml`, `yandex`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: ZeroTrustAccessIdentityProviderArgs,
) -> ZeroTrustAccessIdentityProviderResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let configs_binding = args.configs.get_inner();
    let name_binding = args.name.get_inner();
    let scim_configs_binding = args.scim_configs.get_inner();
    let type__binding = args.type_.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zeroTrustAccessIdentityProvider:ZeroTrustAccessIdentityProvider"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "configs".into(),
                value: &configs_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "scimConfigs".into(),
                value: &scim_configs_binding,
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
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "configs".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "scimConfigs".into() },
            register_interface::ResultField { name : "type".into() },
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
    ZeroTrustAccessIdentityProviderResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        configs: into_domain(hashmap.remove("configs").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        scim_configs: into_domain(hashmap.remove("scimConfigs").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
