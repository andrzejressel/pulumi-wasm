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
pub mod app_image_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppImageConfigArgs {
        /// The name of the App Image Config.
        #[builder(into)]
        pub app_image_config_name: pulumi_wasm_rust::Output<String>,
        /// The CodeEditorAppImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in Code Editor. See Code Editor App Image Config details below.
        #[builder(into, default)]
        pub code_editor_app_image_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::AppImageConfigCodeEditorAppImageConfig,
            >,
        >,
        /// The JupyterLabAppImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in JupyterLab. See Jupyter Lab Image Config details below.
        #[builder(into, default)]
        pub jupyter_lab_image_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfig>,
        >,
        /// The configuration for the file system and kernels in a SageMaker image running as a KernelGateway app. See Kernel Gateway Image Config details below.
        #[builder(into, default)]
        pub kernel_gateway_image_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::AppImageConfigKernelGatewayImageConfig,
            >,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppImageConfigResult {
        /// The name of the App Image Config.
        pub app_image_config_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) assigned by AWS to this App Image Config.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The CodeEditorAppImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in Code Editor. See Code Editor App Image Config details below.
        pub code_editor_app_image_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::AppImageConfigCodeEditorAppImageConfig,
            >,
        >,
        /// The JupyterLabAppImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in JupyterLab. See Jupyter Lab Image Config details below.
        pub jupyter_lab_image_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::AppImageConfigJupyterLabImageConfig>,
        >,
        /// The configuration for the file system and kernels in a SageMaker image running as a KernelGateway app. See Kernel Gateway Image Config details below.
        pub kernel_gateway_image_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::AppImageConfigKernelGatewayImageConfig,
            >,
        >,
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
    pub fn create(name: &str, args: AppImageConfigArgs) -> AppImageConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_image_config_name_binding = args.app_image_config_name.get_inner();
        let code_editor_app_image_config_binding = args
            .code_editor_app_image_config
            .get_inner();
        let jupyter_lab_image_config_binding = args.jupyter_lab_image_config.get_inner();
        let kernel_gateway_image_config_binding = args
            .kernel_gateway_image_config
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/appImageConfig:AppImageConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appImageConfigName".into(),
                    value: &app_image_config_name_binding,
                },
                register_interface::ObjectField {
                    name: "codeEditorAppImageConfig".into(),
                    value: &code_editor_app_image_config_binding,
                },
                register_interface::ObjectField {
                    name: "jupyterLabImageConfig".into(),
                    value: &jupyter_lab_image_config_binding,
                },
                register_interface::ObjectField {
                    name: "kernelGatewayImageConfig".into(),
                    value: &kernel_gateway_image_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appImageConfigName".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "codeEditorAppImageConfig".into(),
                },
                register_interface::ResultField {
                    name: "jupyterLabImageConfig".into(),
                },
                register_interface::ResultField {
                    name: "kernelGatewayImageConfig".into(),
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
        AppImageConfigResult {
            app_image_config_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appImageConfigName").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            code_editor_app_image_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("codeEditorAppImageConfig").unwrap(),
            ),
            jupyter_lab_image_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jupyterLabImageConfig").unwrap(),
            ),
            kernel_gateway_image_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kernelGatewayImageConfig").unwrap(),
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
