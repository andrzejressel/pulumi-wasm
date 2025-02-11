/// Provides a SageMaker Studio Lifecycle Config resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sagemaker:StudioLifecycleConfig
///     properties:
///       studioLifecycleConfigName: example
///       studioLifecycleConfigAppType: JupyterServer
///       studioLifecycleConfigContent:
///         fn::invoke:
///           function: std:base64encode
///           arguments:
///             input: echo Hello
///           return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Studio Lifecycle Configs using the `studio_lifecycle_config_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/studioLifecycleConfig:StudioLifecycleConfig example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod studio_lifecycle_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StudioLifecycleConfigArgs {
        /// The App type that the Lifecycle Configuration is attached to. Valid values are `JupyterServer`, `JupyterLab`, `CodeEditor` and `KernelGateway`.
        #[builder(into)]
        pub studio_lifecycle_config_app_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The content of your Studio Lifecycle Configuration script. This content must be base64 encoded.
        #[builder(into)]
        pub studio_lifecycle_config_content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Studio Lifecycle Configuration to create.
        #[builder(into)]
        pub studio_lifecycle_config_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StudioLifecycleConfigResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Studio Lifecycle Config.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The App type that the Lifecycle Configuration is attached to. Valid values are `JupyterServer`, `JupyterLab`, `CodeEditor` and `KernelGateway`.
        pub studio_lifecycle_config_app_type: pulumi_gestalt_rust::Output<String>,
        /// The content of your Studio Lifecycle Configuration script. This content must be base64 encoded.
        pub studio_lifecycle_config_content: pulumi_gestalt_rust::Output<String>,
        /// The name of the Studio Lifecycle Configuration to create.
        pub studio_lifecycle_config_name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StudioLifecycleConfigArgs,
    ) -> StudioLifecycleConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let studio_lifecycle_config_app_type_binding = args
            .studio_lifecycle_config_app_type
            .get_output(context);
        let studio_lifecycle_config_content_binding = args
            .studio_lifecycle_config_content
            .get_output(context);
        let studio_lifecycle_config_name_binding = args
            .studio_lifecycle_config_name
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/studioLifecycleConfig:StudioLifecycleConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "studioLifecycleConfigAppType".into(),
                    value: &studio_lifecycle_config_app_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "studioLifecycleConfigContent".into(),
                    value: &studio_lifecycle_config_content_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "studioLifecycleConfigName".into(),
                    value: &studio_lifecycle_config_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StudioLifecycleConfigResult {
            arn: o.get_field("arn"),
            studio_lifecycle_config_app_type: o
                .get_field("studioLifecycleConfigAppType"),
            studio_lifecycle_config_content: o.get_field("studioLifecycleConfigContent"),
            studio_lifecycle_config_name: o.get_field("studioLifecycleConfigName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
