/// Configurations options for the tenant for authenticating with a the standard set of Identity Toolkit-trusted IDPs.
///
/// You must enable the
/// [Google Identity Platform](https://console.cloud.google.com/marketplace/details/google-cloud-platform/customer-identity) in
/// the marketplace prior to using this resource.
///
///
///
/// ## Example Usage
///
/// ### Identity Platform Tenant Default Supported Idp Config Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let idpConfig = tenant_default_supported_idp_config::create(
///         "idpConfig",
///         TenantDefaultSupportedIdpConfigArgs::builder()
///             .client_id("my-client-id")
///             .client_secret("secret")
///             .enabled(true)
///             .idp_id("playgames.google.com")
///             .tenant("${tenant.name}")
///             .build_struct(),
///     );
///     let tenant = tenant::create(
///         "tenant",
///         TenantArgs::builder().display_name("tenant").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TenantDefaultSupportedIdpConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/tenants/{{tenant}}/defaultSupportedIdpConfigs/{{idp_id}}`
///
/// * `{{project}}/{{tenant}}/{{idp_id}}`
///
/// * `{{tenant}}/{{idp_id}}`
///
/// When using the `pulumi import` command, TenantDefaultSupportedIdpConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenantDefaultSupportedIdpConfig:TenantDefaultSupportedIdpConfig default projects/{{project}}/tenants/{{tenant}}/defaultSupportedIdpConfigs/{{idp_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenantDefaultSupportedIdpConfig:TenantDefaultSupportedIdpConfig default {{project}}/{{tenant}}/{{idp_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenantDefaultSupportedIdpConfig:TenantDefaultSupportedIdpConfig default {{tenant}}/{{idp_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod tenant_default_supported_idp_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TenantDefaultSupportedIdpConfigArgs {
        /// OAuth client ID
        #[builder(into)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// OAuth client secret
        ///
        ///
        /// - - -
        #[builder(into)]
        pub client_secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If this IDP allows the user to sign in
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ID of the IDP. Possible values include:
        /// * `apple.com`
        /// * `facebook.com`
        /// * `gc.apple.com`
        /// * `github.com`
        /// * `google.com`
        /// * `linkedin.com`
        /// * `microsoft.com`
        /// * `playgames.google.com`
        /// * `twitter.com`
        /// * `yahoo.com`
        #[builder(into)]
        pub idp_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the tenant where this DefaultSupportedIdpConfig resource exists
        #[builder(into)]
        pub tenant: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TenantDefaultSupportedIdpConfigResult {
        /// OAuth client ID
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// OAuth client secret
        ///
        ///
        /// - - -
        pub client_secret: pulumi_gestalt_rust::Output<String>,
        /// If this IDP allows the user to sign in
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ID of the IDP. Possible values include:
        /// * `apple.com`
        /// * `facebook.com`
        /// * `gc.apple.com`
        /// * `github.com`
        /// * `google.com`
        /// * `linkedin.com`
        /// * `microsoft.com`
        /// * `playgames.google.com`
        /// * `twitter.com`
        /// * `yahoo.com`
        pub idp_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the default supported IDP config resource
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The name of the tenant where this DefaultSupportedIdpConfig resource exists
        pub tenant: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TenantDefaultSupportedIdpConfigArgs,
    ) -> TenantDefaultSupportedIdpConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let client_id_binding = args.client_id.get_output(context).get_inner();
        let client_secret_binding = args.client_secret.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let idp_id_binding = args.idp_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let tenant_binding = args.tenant.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:identityplatform/tenantDefaultSupportedIdpConfig:TenantDefaultSupportedIdpConfig"
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
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "idpId".into(),
                    value: &idp_id_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TenantDefaultSupportedIdpConfigResult {
            client_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            client_secret: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientSecret"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            idp_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("idpId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            tenant: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenant"),
            ),
        }
    }
}
