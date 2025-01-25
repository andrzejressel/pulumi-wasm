/// Provides a SageMaker App resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app::create(
///         "example",
///         AppArgs::builder()
///             .app_name("example")
///             .app_type("JupyterServer")
///             .domain_id("${exampleAwsSagemakerDomain.id}")
///             .user_profile_name("${exampleAwsSagemakerUserProfile.userProfileName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Apps using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/app:App example arn:aws:sagemaker:us-west-2:012345678912:app/domain-id/user-profile-name/app-type/app-name
/// ```
pub mod app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppArgs {
        /// The name of the app.
        #[builder(into)]
        pub app_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of app. Valid values are `JupyterServer`, `KernelGateway`, `RStudioServerPro`, `RSessionGateway`, `TensorBoard`, `CodeEditor`, `JupyterLab`, `DetailedProfiler`, and `Canvas`.
        #[builder(into)]
        pub app_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The domain ID.
        #[builder(into)]
        pub domain_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance.See Resource Spec below.
        #[builder(into, default)]
        pub resource_spec: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sagemaker::AppResourceSpec>,
        >,
        /// The name of the space. At least one of `user_profile_name` or `space_name` required.
        #[builder(into, default)]
        pub space_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The user profile name. At least one of `user_profile_name` or `space_name` required.
        #[builder(into, default)]
        pub user_profile_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppResult {
        /// The name of the app.
        pub app_name: pulumi_wasm_rust::Output<String>,
        /// The type of app. Valid values are `JupyterServer`, `KernelGateway`, `RStudioServerPro`, `RSessionGateway`, `TensorBoard`, `CodeEditor`, `JupyterLab`, `DetailedProfiler`, and `Canvas`.
        pub app_type: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the app.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The domain ID.
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// The instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance.See Resource Spec below.
        pub resource_spec: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::AppResourceSpec,
        >,
        /// The name of the space. At least one of `user_profile_name` or `space_name` required.
        pub space_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The user profile name. At least one of `user_profile_name` or `space_name` required.
        pub user_profile_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AppArgs,
    ) -> AppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_name_binding = args.app_name.get_output(context).get_inner();
        let app_type_binding = args.app_type.get_output(context).get_inner();
        let domain_id_binding = args.domain_id.get_output(context).get_inner();
        let resource_spec_binding = args.resource_spec.get_output(context).get_inner();
        let space_name_binding = args.space_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_profile_name_binding = args
            .user_profile_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/app:App".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appName".into(),
                    value: &app_name_binding,
                },
                register_interface::ObjectField {
                    name: "appType".into(),
                    value: &app_type_binding,
                },
                register_interface::ObjectField {
                    name: "domainId".into(),
                    value: &domain_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceSpec".into(),
                    value: &resource_spec_binding,
                },
                register_interface::ObjectField {
                    name: "spaceName".into(),
                    value: &space_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userProfileName".into(),
                    value: &user_profile_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appName".into(),
                },
                register_interface::ResultField {
                    name: "appType".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "domainId".into(),
                },
                register_interface::ResultField {
                    name: "resourceSpec".into(),
                },
                register_interface::ResultField {
                    name: "spaceName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userProfileName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppResult {
            app_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appName").unwrap(),
            ),
            app_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appType").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            domain_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainId").unwrap(),
            ),
            resource_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceSpec").unwrap(),
            ),
            space_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spaceName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userProfileName").unwrap(),
            ),
        }
    }
}
