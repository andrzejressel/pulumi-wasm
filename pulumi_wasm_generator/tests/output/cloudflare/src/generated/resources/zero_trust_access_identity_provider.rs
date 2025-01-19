/// Provides a Cloudflare Access Identity Provider resource. Identity
/// Providers are used as an authentication or authorisation source
/// within Access.
///
/// > It's required that an `account_id` or `zone_id` is provided and in
///    most cases using either is fine. However, if you're using a scoped
///    access token, you must provide the argument that matches the token's
///    scope. For example, an access token that is scoped to the "example.com"
///    zone needs to use the `zone_id` argument.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let githubOauth = zero_trust_access_identity_provider::create(
///         "githubOauth",
///         ZeroTrustAccessIdentityProviderArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .configs(
///                 vec![
///                     ZeroTrustAccessIdentityProviderConfig::builder().clientId("example")
///                     .clientSecret("secret_key").build_struct(),
///                 ],
///             )
///             .name("GitHub OAuth")
///             .type_("github")
///             .build_struct(),
///     );
///     let jumpcloudSaml = zero_trust_access_identity_provider::create(
///         "jumpcloudSaml",
///         ZeroTrustAccessIdentityProviderArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .configs(
///                 vec![
///                     ZeroTrustAccessIdentityProviderConfig::builder()
///                     .attributes(vec!["email", "username",])
///                     .idpPublicCert("MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg\nuTnQyuOqPuHbnN83d/2l1NSYKCbHt24o")
///                     .issuerUrl("jumpcloud").signRequest(false)
///                     .ssoTargetUrl("https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess")
///                     .build_struct(),
///                 ],
///             )
///             .name("JumpCloud SAML")
///             .type_("saml")
///             .build_struct(),
///     );
///     let okta = zero_trust_access_identity_provider::create(
///         "okta",
///         ZeroTrustAccessIdentityProviderArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .configs(
///                 vec![
///                     ZeroTrustAccessIdentityProviderConfig::builder()
///                     .apiToken("okta_api_token").clientId("example")
///                     .clientSecret("secret_key").oktaAccount("https://example.com")
///                     .build_struct(),
///                 ],
///             )
///             .name("Okta")
///             .type_("okta")
///             .build_struct(),
///     );
///     let pinLogin = zero_trust_access_identity_provider::create(
///         "pinLogin",
///         ZeroTrustAccessIdentityProviderArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .name("PIN login")
///             .type_("onetimepin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustAccessIdentityProvider:ZeroTrustAccessIdentityProvider example <account_id>/<identity_provider_id>
/// ```
///
pub mod zero_trust_access_identity_provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ZeroTrustAccessIdentityProviderArgs,
    ) -> ZeroTrustAccessIdentityProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            version: super::get_version(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "configs".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "scimConfigs".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZeroTrustAccessIdentityProviderResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configs").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            scim_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scimConfigs").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
