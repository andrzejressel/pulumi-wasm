/// Provides a CodeDeploy application to be used as a basis for deployments
///
/// ## Example Usage
///
/// ### ECS Application
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder().compute_platform("ECS").name("example").build_struct(),
///     );
/// }
/// ```
///
/// ### Lambda Application
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder()
///             .compute_platform("Lambda")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Server Application
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder()
///             .compute_platform("Server")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeDeploy Applications using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:codedeploy/application:Application example my-application
/// ```
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The compute platform can either be `ECS`, `Lambda`, or `Server`. Default is `Server`.
        #[builder(into, default)]
        pub compute_platform: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the application.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// The application ID.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the CodeDeploy application.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The compute platform can either be `ECS`, `Lambda`, or `Server`. Default is `Server`.
        pub compute_platform: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for a connection to a GitHub account.
        pub github_account_name: pulumi_wasm_rust::Output<String>,
        /// Whether the user has authenticated with GitHub for the specified application.
        pub linked_to_github: pulumi_wasm_rust::Output<bool>,
        /// The name of the application.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        let compute_platform_binding = args.compute_platform.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codedeploy/application:Application".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "computePlatform".into(),
                    value: &compute_platform_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "computePlatform".into(),
                },
                register_interface::ResultField {
                    name: "githubAccountName".into(),
                },
                register_interface::ResultField {
                    name: "linkedToGithub".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            compute_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computePlatform").unwrap(),
            ),
            github_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("githubAccountName").unwrap(),
            ),
            linked_to_github: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedToGithub").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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