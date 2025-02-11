/// Provides a SageMaker App Image Config resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:sagemaker:AppImageConfig
///     properties:
///       appImageConfigName: example
///       kernelGatewayImageConfig:
///         kernelSpecs:
///           - name: example
/// ```
///
/// ### Default File System Config
///
/// ```yaml
/// resources:
///   test:
///     type: aws:sagemaker:AppImageConfig
///     properties:
///       appImageConfigName: example
///       kernelGatewayImageConfig:
///         kernelSpecs:
///           - name: example
///         fileSystemConfig: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker App Image Configs using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/appImageConfig:AppImageConfig example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_image_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppImageConfigArgs {
        /// The name of the App Image Config.
        #[builder(into)]
        pub app_image_config_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The CodeEditorAppImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in Code Editor. See Code Editor App Image Config details below.
        #[builder(into, default)]
        pub code_editor_app_image_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::sagemaker::AppImageConfigCodeEditorAppImageConfig,
            >,
        >,
        /// The JupyterLabAppImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in JupyterLab. See Jupyter Lab Image Config details below.
        #[builder(into, default)]
        pub jupyter_lab_image_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfig>,
        >,
        /// The configuration for the file system and kernels in a SageMaker image running as a KernelGateway app. See Kernel Gateway Image Config details below.
        #[builder(into, default)]
        pub kernel_gateway_image_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::sagemaker::AppImageConfigKernelGatewayImageConfig,
            >,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppImageConfigResult {
        /// The name of the App Image Config.
        pub app_image_config_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) assigned by AWS to this App Image Config.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The CodeEditorAppImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in Code Editor. See Code Editor App Image Config details below.
        pub code_editor_app_image_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::sagemaker::AppImageConfigCodeEditorAppImageConfig,
            >,
        >,
        /// The JupyterLabAppImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in JupyterLab. See Jupyter Lab Image Config details below.
        pub jupyter_lab_image_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfig>,
        >,
        /// The configuration for the file system and kernels in a SageMaker image running as a KernelGateway app. See Kernel Gateway Image Config details below.
        pub kernel_gateway_image_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::sagemaker::AppImageConfigKernelGatewayImageConfig,
            >,
        >,
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
        args: AppImageConfigArgs,
    ) -> AppImageConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_image_config_name_binding = args
            .app_image_config_name
            .get_output(context);
        let code_editor_app_image_config_binding = args
            .code_editor_app_image_config
            .get_output(context);
        let jupyter_lab_image_config_binding = args
            .jupyter_lab_image_config
            .get_output(context);
        let kernel_gateway_image_config_binding = args
            .kernel_gateway_image_config
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/appImageConfig:AppImageConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appImageConfigName".into(),
                    value: &app_image_config_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "codeEditorAppImageConfig".into(),
                    value: &code_editor_app_image_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jupyterLabImageConfig".into(),
                    value: &jupyter_lab_image_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kernelGatewayImageConfig".into(),
                    value: &kernel_gateway_image_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppImageConfigResult {
            app_image_config_name: o.get_field("appImageConfigName"),
            arn: o.get_field("arn"),
            code_editor_app_image_config: o.get_field("codeEditorAppImageConfig"),
            jupyter_lab_image_config: o.get_field("jupyterLabImageConfig"),
            kernel_gateway_image_config: o.get_field("kernelGatewayImageConfig"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
