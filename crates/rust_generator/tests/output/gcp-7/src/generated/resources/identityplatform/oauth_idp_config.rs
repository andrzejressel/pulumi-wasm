/// OIDC IdP configuration for a Identity Toolkit project.
///
/// You must enable the
/// [Google Identity Platform](https://console.cloud.google.com/marketplace/details/google-cloud-platform/customer-identity) in
/// the marketplace prior to using this resource.
///
///
///
/// ## Example Usage
///
/// ### Identity Platform Oauth Idp Config Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let oauthIdpConfig = oauth_idp_config::create(
///         "oauthIdpConfig",
///         OauthIdpConfigArgs::builder()
///             .client_id("client-id")
///             .client_secret("secret")
///             .display_name("Display Name")
///             .enabled(true)
///             .issuer("issuer")
///             .name("oidc.oauth-idp-config")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OauthIdpConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/oauthIdpConfigs/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, OauthIdpConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:identityplatform/oauthIdpConfig:OauthIdpConfig default projects/{{project}}/oauthIdpConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/oauthIdpConfig:OauthIdpConfig default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/oauthIdpConfig:OauthIdpConfig default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod oauth_idp_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OauthIdpConfigArgs {
        /// The client id of an OAuth client.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The client secret of the OAuth client, to enable OIDC code flow.
        #[builder(into, default)]
        pub client_secret: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Human friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If this config allows users to sign in with the provider.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// For OIDC Idps, the issuer identifier.
        #[builder(into)]
        pub issuer: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the OauthIdpConfig. Must start with `oidc.`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OauthIdpConfigResult {
        /// The client id of an OAuth client.
        ///
        ///
        /// - - -
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// The client secret of the OAuth client, to enable OIDC code flow.
        pub client_secret: pulumi_gestalt_rust::Output<Option<String>>,
        /// Human friendly display name.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// If this config allows users to sign in with the provider.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// For OIDC Idps, the issuer identifier.
        pub issuer: pulumi_gestalt_rust::Output<String>,
        /// The name of the OauthIdpConfig. Must start with `oidc.`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: OauthIdpConfigArgs,
    ) -> OauthIdpConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let client_id_binding_1 = args.client_id.get_output(context);
        let client_id_binding = client_id_binding_1.get_inner();
        let client_secret_binding_1 = args.client_secret.get_output(context);
        let client_secret_binding = client_secret_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let issuer_binding_1 = args.issuer.get_output(context);
        let issuer_binding = issuer_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:identityplatform/oauthIdpConfig:OauthIdpConfig".into(),
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OauthIdpConfigResult {
            client_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            client_secret: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientSecret"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            issuer: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("issuer"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
        }
    }
}
