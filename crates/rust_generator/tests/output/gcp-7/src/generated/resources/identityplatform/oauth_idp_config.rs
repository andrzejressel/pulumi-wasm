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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OauthIdpConfigArgs,
    ) -> OauthIdpConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let client_id_binding = args.client_id.get_output(context);
        let client_secret_binding = args.client_secret.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let issuer_binding = args.issuer.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:identityplatform/oauthIdpConfig:OauthIdpConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientSecret".into(),
                    value: &client_secret_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "issuer".into(),
                    value: &issuer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OauthIdpConfigResult {
            client_id: o.get_field("clientId"),
            client_secret: o.get_field("clientSecret"),
            display_name: o.get_field("displayName"),
            enabled: o.get_field("enabled"),
            issuer: o.get_field("issuer"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
