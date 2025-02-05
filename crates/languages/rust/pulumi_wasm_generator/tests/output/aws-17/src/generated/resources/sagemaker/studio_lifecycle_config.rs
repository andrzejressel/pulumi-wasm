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
pub mod studio_lifecycle_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StudioLifecycleConfigArgs {
        /// The App type that the Lifecycle Configuration is attached to. Valid values are `JupyterServer`, `JupyterLab`, `CodeEditor` and `KernelGateway`.
        #[builder(into)]
        pub studio_lifecycle_config_app_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The content of your Studio Lifecycle Configuration script. This content must be base64 encoded.
        #[builder(into)]
        pub studio_lifecycle_config_content: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Studio Lifecycle Configuration to create.
        #[builder(into)]
        pub studio_lifecycle_config_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StudioLifecycleConfigResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Studio Lifecycle Config.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The App type that the Lifecycle Configuration is attached to. Valid values are `JupyterServer`, `JupyterLab`, `CodeEditor` and `KernelGateway`.
        pub studio_lifecycle_config_app_type: pulumi_wasm_rust::Output<String>,
        /// The content of your Studio Lifecycle Configuration script. This content must be base64 encoded.
        pub studio_lifecycle_config_content: pulumi_wasm_rust::Output<String>,
        /// The name of the Studio Lifecycle Configuration to create.
        pub studio_lifecycle_config_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StudioLifecycleConfigArgs,
    ) -> StudioLifecycleConfigResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let studio_lifecycle_config_app_type_binding = args
            .studio_lifecycle_config_app_type
            .get_output(context)
            .get_inner();
        let studio_lifecycle_config_content_binding = args
            .studio_lifecycle_config_content
            .get_output(context)
            .get_inner();
        let studio_lifecycle_config_name_binding = args
            .studio_lifecycle_config_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/studioLifecycleConfig:StudioLifecycleConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "studioLifecycleConfigAppType".into(),
                    value: &studio_lifecycle_config_app_type_binding,
                },
                register_interface::ObjectField {
                    name: "studioLifecycleConfigContent".into(),
                    value: &studio_lifecycle_config_content_binding,
                },
                register_interface::ObjectField {
                    name: "studioLifecycleConfigName".into(),
                    value: &studio_lifecycle_config_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StudioLifecycleConfigResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            studio_lifecycle_config_app_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("studioLifecycleConfigAppType"),
            ),
            studio_lifecycle_config_content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("studioLifecycleConfigContent"),
            ),
            studio_lifecycle_config_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("studioLifecycleConfigName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
