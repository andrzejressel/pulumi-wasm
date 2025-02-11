/// Provides a SageMaker App resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppArgs {
        /// The name of the app.
        #[builder(into)]
        pub app_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of app. Valid values are `JupyterServer`, `KernelGateway`, `RStudioServerPro`, `RSessionGateway`, `TensorBoard`, `CodeEditor`, `JupyterLab`, `DetailedProfiler`, and `Canvas`.
        #[builder(into)]
        pub app_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The domain ID.
        #[builder(into)]
        pub domain_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance.See Resource Spec below.
        #[builder(into, default)]
        pub resource_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::AppResourceSpec>,
        >,
        /// The name of the space. At least one of `user_profile_name` or `space_name` required.
        #[builder(into, default)]
        pub space_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The user profile name. At least one of `user_profile_name` or `space_name` required.
        #[builder(into, default)]
        pub user_profile_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppResult {
        /// The name of the app.
        pub app_name: pulumi_gestalt_rust::Output<String>,
        /// The type of app. Valid values are `JupyterServer`, `KernelGateway`, `RStudioServerPro`, `RSessionGateway`, `TensorBoard`, `CodeEditor`, `JupyterLab`, `DetailedProfiler`, and `Canvas`.
        pub app_type: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the app.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The domain ID.
        pub domain_id: pulumi_gestalt_rust::Output<String>,
        /// The instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance.See Resource Spec below.
        pub resource_spec: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::AppResourceSpec,
        >,
        /// The name of the space. At least one of `user_profile_name` or `space_name` required.
        pub space_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The user profile name. At least one of `user_profile_name` or `space_name` required.
        pub user_profile_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppArgs,
    ) -> AppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_name_binding = args.app_name.get_output(context);
        let app_type_binding = args.app_type.get_output(context);
        let domain_id_binding = args.domain_id.get_output(context);
        let resource_spec_binding = args.resource_spec.get_output(context);
        let space_name_binding = args.space_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_profile_name_binding = args.user_profile_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/app:App".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appName".into(),
                    value: &app_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appType".into(),
                    value: &app_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainId".into(),
                    value: &domain_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceSpec".into(),
                    value: &resource_spec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spaceName".into(),
                    value: &space_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userProfileName".into(),
                    value: &user_profile_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppResult {
            app_name: o.get_field("appName"),
            app_type: o.get_field("appType"),
            arn: o.get_field("arn"),
            domain_id: o.get_field("domainId"),
            resource_spec: o.get_field("resourceSpec"),
            space_name: o.get_field("spaceName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_profile_name: o.get_field("userProfileName"),
        }
    }
}
