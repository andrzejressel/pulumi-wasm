/// Resource for managing an AWS SSO Admin Application.
///
/// > The `CreateApplication` API only supports custom OAuth 2.0 applications.
/// Creation of 3rd party SAML or OAuth 2.0 applications require setup to be done through the associated app service or AWS console.
/// See this issue for additional context.
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
///     let example = get_instances::invoke(GetInstancesArgs::builder().build_struct());
///     let exampleApplication = application::create(
///         "exampleApplication",
///         ApplicationArgs::builder()
///             .application_provider_arn("arn:aws:sso::aws:applicationProvider/custom")
///             .instance_arn("${example.arns[0]}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Portal Options
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_instances::invoke(GetInstancesArgs::builder().build_struct());
///     let exampleApplication = application::create(
///         "exampleApplication",
///         ApplicationArgs::builder()
///             .application_provider_arn("arn:aws:sso::aws:applicationProvider/custom")
///             .instance_arn("${example.arns[0]}")
///             .name("example")
///             .portal_options(
///                 ApplicationPortalOptions::builder()
///                     .signInOptions(
///                         ApplicationPortalOptionsSignInOptions::builder()
///                             .applicationUrl("http://example.com")
///                             .origin("APPLICATION")
///                             .build_struct(),
///                     )
///                     .visibility("ENABLED")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Admin Application using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/application:Application example arn:aws:sso::123456789012:application/id-12345678
/// ```
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// ARN of the application provider.
        #[builder(into)]
        pub application_provider_arn: pulumi_wasm_rust::Output<String>,
        /// A unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
        #[builder(into, default)]
        pub client_token: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the application.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the instance of IAM Identity Center.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the application.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Options for the portal associated with an application. See `portal_options` below.
        #[builder(into, default)]
        pub portal_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ssoadmin::ApplicationPortalOptions>,
        >,
        /// Status of the application. Valid values are `ENABLED` and `DISABLED`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// AWS account ID.
        pub application_account: pulumi_wasm_rust::Output<String>,
        /// ARN of the application.
        pub application_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the application provider.
        pub application_provider_arn: pulumi_wasm_rust::Output<String>,
        /// A unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
        pub client_token: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the application.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the instance of IAM Identity Center.
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the application.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Options for the portal associated with an application. See `portal_options` below.
        pub portal_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ssoadmin::ApplicationPortalOptions>,
        >,
        /// Status of the application. Valid values are `ENABLED` and `DISABLED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationArgs) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_provider_arn_binding = args.application_provider_arn.get_inner();
        let client_token_binding = args.client_token.get_inner();
        let description_binding = args.description.get_inner();
        let instance_arn_binding = args.instance_arn.get_inner();
        let name_binding = args.name.get_inner();
        let portal_options_binding = args.portal_options.get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/application:Application".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationProviderArn".into(),
                    value: &application_provider_arn_binding,
                },
                register_interface::ObjectField {
                    name: "clientToken".into(),
                    value: &client_token_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "portalOptions".into(),
                    value: &portal_options_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationAccount".into(),
                },
                register_interface::ResultField {
                    name: "applicationArn".into(),
                },
                register_interface::ResultField {
                    name: "applicationProviderArn".into(),
                },
                register_interface::ResultField {
                    name: "clientToken".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "instanceArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "portalOptions".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationResult {
            application_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationAccount").unwrap(),
            ),
            application_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationArn").unwrap(),
            ),
            application_provider_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationProviderArn").unwrap(),
            ),
            client_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientToken").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            portal_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portalOptions").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}