/// Resource for managing an AWS AppFabric App Authorization.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod app_authorization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppAuthorizationArgs {
        /// The name of the application for valid values see https://docs.aws.amazon.com/appfabric/latest/api/API_CreateAppAuthorization.html.
        #[builder(into)]
        pub app: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        #[builder(into)]
        pub app_bundle_arn: pulumi_wasm_rust::Output<String>,
        /// The authorization type for the app authorization valid values are oauth2 and apiKey.
        #[builder(into)]
        pub auth_type: pulumi_wasm_rust::Output<String>,
        /// Contains credentials for the application, such as an API key or OAuth2 client ID and secret.
        /// Specify credentials that match the authorization type for your request. For example, if the authorization type for your request is OAuth2 (oauth2), then you should provide only the OAuth2 credentials.
        #[builder(into, default)]
        pub credential: pulumi_wasm_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationCredential>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Contains information about an application tenant, such as the application display name and identifier.
        #[builder(into, default)]
        pub tenants: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appfabric::AppAuthorizationTenant>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppAuthorizationResult {
        /// The name of the application for valid values see https://docs.aws.amazon.com/appfabric/latest/api/API_CreateAppAuthorization.html.
        pub app: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        pub app_bundle_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the App Authorization. Do not begin the description with "An", "The", "Defines", "Indicates", or "Specifies," as these are verbose. In other words, "Indicates the amount of storage," can be rewritten as "Amount of storage," without losing any information.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The authorization type for the app authorization valid values are oauth2 and apiKey.
        pub auth_type: pulumi_wasm_rust::Output<String>,
        /// The application URL for the OAuth flow.
        pub auth_url: pulumi_wasm_rust::Output<String>,
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Contains credentials for the application, such as an API key or OAuth2 client ID and secret.
        /// Specify credentials that match the authorization type for your request. For example, if the authorization type for your request is OAuth2 (oauth2), then you should provide only the OAuth2 credentials.
        pub credential: pulumi_wasm_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationCredential>,
        >,
        /// The user persona of the app authorization.
        pub persona: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Contains information about an application tenant, such as the application display name and identifier.
        pub tenants: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appfabric::AppAuthorizationTenant>>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationTimeouts>,
        >,
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AppAuthorizationArgs) -> AppAuthorizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_binding = args.app.get_inner();
        let app_bundle_arn_binding = args.app_bundle_arn.get_inner();
        let auth_type_binding = args.auth_type.get_inner();
        let credential_binding = args.credential.get_inner();
        let tags_binding = args.tags.get_inner();
        let tenants_binding = args.tenants.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appfabric/appAuthorization:AppAuthorization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "app".into(),
                    value: &app_binding,
                },
                register_interface::ObjectField {
                    name: "appBundleArn".into(),
                    value: &app_bundle_arn_binding,
                },
                register_interface::ObjectField {
                    name: "authType".into(),
                    value: &auth_type_binding,
                },
                register_interface::ObjectField {
                    name: "credential".into(),
                    value: &credential_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tenants".into(),
                    value: &tenants_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "app".into(),
                },
                register_interface::ResultField {
                    name: "appBundleArn".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authType".into(),
                },
                register_interface::ResultField {
                    name: "authUrl".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "credential".into(),
                },
                register_interface::ResultField {
                    name: "persona".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tenants".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppAuthorizationResult {
            app: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("app").unwrap(),
            ),
            app_bundle_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appBundleArn").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auth_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authType").unwrap(),
            ),
            auth_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authUrl").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            credential: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credential").unwrap(),
            ),
            persona: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("persona").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tenants: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenants").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}
