/// Resource for managing an AWS AppFabric App Authorization.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app_authorization::create(
///         "example",
///         AppAuthorizationArgs::builder()
///             .app("TERRAFORMCLOUD")
///             .app_bundle_arn("${arn}")
///             .auth_type("apiKey")
///             .credential(
///                 AppAuthorizationCredential::builder()
///                     .apiKeyCredentials(
///                         vec![
///                             AppAuthorizationCredentialApiKeyCredential::builder()
///                             .apiKey("exampleapikeytoken").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .tenants(
///                 vec![
///                     AppAuthorizationTenant::builder().tenantDisplayName("example")
///                     .tenantIdentifier("example").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_authorization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppAuthorizationArgs {
        /// The name of the application for valid values see https://docs.aws.amazon.com/appfabric/latest/api/API_CreateAppAuthorization.html.
        #[builder(into)]
        pub app: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        #[builder(into)]
        pub app_bundle_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The authorization type for the app authorization valid values are oauth2 and apiKey.
        #[builder(into)]
        pub auth_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Contains credentials for the application, such as an API key or OAuth2 client ID and secret.
        /// Specify credentials that match the authorization type for your request. For example, if the authorization type for your request is OAuth2 (oauth2), then you should provide only the OAuth2 credentials.
        #[builder(into, default)]
        pub credential: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appfabric::AppAuthorizationCredential>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Contains information about an application tenant, such as the application display name and identifier.
        #[builder(into, default)]
        pub tenants: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appfabric::AppAuthorizationTenant>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appfabric::AppAuthorizationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppAuthorizationResult {
        /// The name of the application for valid values see https://docs.aws.amazon.com/appfabric/latest/api/API_CreateAppAuthorization.html.
        pub app: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        pub app_bundle_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the App Authorization. Do not begin the description with "An", "The", "Defines", "Indicates", or "Specifies," as these are verbose. In other words, "Indicates the amount of storage," can be rewritten as "Amount of storage," without losing any information.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The authorization type for the app authorization valid values are oauth2 and apiKey.
        pub auth_type: pulumi_gestalt_rust::Output<String>,
        /// The application URL for the OAuth flow.
        pub auth_url: pulumi_gestalt_rust::Output<String>,
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Contains credentials for the application, such as an API key or OAuth2 client ID and secret.
        /// Specify credentials that match the authorization type for your request. For example, if the authorization type for your request is OAuth2 (oauth2), then you should provide only the OAuth2 credentials.
        pub credential: pulumi_gestalt_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationCredential>,
        >,
        /// The user persona of the app authorization.
        pub persona: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Contains information about an application tenant, such as the application display name and identifier.
        pub tenants: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::appfabric::AppAuthorizationTenant>>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationTimeouts>,
        >,
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppAuthorizationArgs,
    ) -> AppAuthorizationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_binding = args.app.get_output(context);
        let app_bundle_arn_binding = args.app_bundle_arn.get_output(context);
        let auth_type_binding = args.auth_type.get_output(context);
        let credential_binding = args.credential.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tenants_binding = args.tenants.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appfabric/appAuthorization:AppAuthorization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "app".into(),
                    value: &app_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appBundleArn".into(),
                    value: &app_bundle_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authType".into(),
                    value: &auth_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "credential".into(),
                    value: &credential_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenants".into(),
                    value: &tenants_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppAuthorizationResult {
            app: o.get_field("app"),
            app_bundle_arn: o.get_field("appBundleArn"),
            arn: o.get_field("arn"),
            auth_type: o.get_field("authType"),
            auth_url: o.get_field("authUrl"),
            created_at: o.get_field("createdAt"),
            credential: o.get_field("credential"),
            persona: o.get_field("persona"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tenants: o.get_field("tenants"),
            timeouts: o.get_field("timeouts"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
