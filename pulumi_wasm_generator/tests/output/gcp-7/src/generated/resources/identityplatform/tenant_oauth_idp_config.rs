/// OIDC IdP configuration for a Identity Toolkit project within a tenant.
///
/// You must enable the
/// [Google Identity Platform](https://console.cloud.google.com/marketplace/details/google-cloud-platform/customer-identity) in
/// the marketplace prior to using this resource.
///
///
///
/// ## Example Usage
///
/// ### Identity Platform Tenant Oauth Idp Config Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let tenant = tenant::create(
///         "tenant",
///         TenantArgs::builder().display_name("tenant").build_struct(),
///     );
///     let tenantOauthIdpConfig = tenant_oauth_idp_config::create(
///         "tenantOauthIdpConfig",
///         TenantOauthIdpConfigArgs::builder()
///             .client_id("client-id")
///             .client_secret("secret")
///             .display_name("Display Name")
///             .enabled(true)
///             .issuer("issuer")
///             .name("oidc.oauth-idp-config")
///             .tenant("${tenant.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TenantOauthIdpConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/tenants/{{tenant}}/oauthIdpConfigs/{{name}}`
///
/// * `{{project}}/{{tenant}}/{{name}}`
///
/// * `{{tenant}}/{{name}}`
///
/// When using the `pulumi import` command, TenantOauthIdpConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenantOauthIdpConfig:TenantOauthIdpConfig default projects/{{project}}/tenants/{{tenant}}/oauthIdpConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenantOauthIdpConfig:TenantOauthIdpConfig default {{project}}/{{tenant}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenantOauthIdpConfig:TenantOauthIdpConfig default {{tenant}}/{{name}}
/// ```
///
pub mod tenant_oauth_idp_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TenantOauthIdpConfigArgs {
        /// The client id of an OAuth client.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The client secret of the OAuth client, to enable OIDC code flow.
        #[builder(into, default)]
        pub client_secret: pulumi_wasm_rust::Output<Option<String>>,
        /// Human friendly display name.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// If this config allows users to sign in with the provider.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// For OIDC Idps, the issuer identifier.
        #[builder(into)]
        pub issuer: pulumi_wasm_rust::Output<String>,
        /// The name of the OauthIdpConfig. Must start with `oidc.`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the tenant where this OIDC IDP configuration resource exists
        #[builder(into)]
        pub tenant: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TenantOauthIdpConfigResult {
        /// The client id of an OAuth client.
        ///
        ///
        /// - - -
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The client secret of the OAuth client, to enable OIDC code flow.
        pub client_secret: pulumi_wasm_rust::Output<Option<String>>,
        /// Human friendly display name.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// If this config allows users to sign in with the provider.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// For OIDC Idps, the issuer identifier.
        pub issuer: pulumi_wasm_rust::Output<String>,
        /// The name of the OauthIdpConfig. Must start with `oidc.`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The name of the tenant where this OIDC IDP configuration resource exists
        pub tenant: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TenantOauthIdpConfigArgs,
    ) -> TenantOauthIdpConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_id_binding = args.client_id.get_inner();
        let client_secret_binding = args.client_secret.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let issuer_binding = args.issuer.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let tenant_binding = args.tenant.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:identityplatform/tenantOauthIdpConfig:TenantOauthIdpConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientSecret".into(),
                    value: &client_secret_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "issuer".into(),
                    value: &issuer_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "tenant".into(),
                    value: &tenant_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clientId".into(),
                },
                register_interface::ResultField {
                    name: "clientSecret".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "issuer".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "tenant".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TenantOauthIdpConfigResult {
            client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientId").unwrap(),
            ),
            client_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientSecret").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            issuer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issuer").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            tenant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenant").unwrap(),
            ),
        }
    }
}
