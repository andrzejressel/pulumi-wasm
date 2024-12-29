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
///           Function: std:base64encode
///           Arguments:
///             input: echo Hello
///           Return: result
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StudioLifecycleConfigArgs {
        /// The App type that the Lifecycle Configuration is attached to. Valid values are `JupyterServer`, `JupyterLab`, `CodeEditor` and `KernelGateway`.
        #[builder(into)]
        pub studio_lifecycle_config_app_type: pulumi_wasm_rust::Output<String>,
        /// The content of your Studio Lifecycle Configuration script. This content must be base64 encoded.
        #[builder(into)]
        pub studio_lifecycle_config_content: pulumi_wasm_rust::Output<String>,
        /// The name of the Studio Lifecycle Configuration to create.
        #[builder(into)]
        pub studio_lifecycle_config_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
        name: &str,
        args: StudioLifecycleConfigArgs,
    ) -> StudioLifecycleConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let studio_lifecycle_config_app_type_binding = args
            .studio_lifecycle_config_app_type
            .get_inner();
        let studio_lifecycle_config_content_binding = args
            .studio_lifecycle_config_content
            .get_inner();
        let studio_lifecycle_config_name_binding = args
            .studio_lifecycle_config_name
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/studioLifecycleConfig:StudioLifecycleConfig".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "studioLifecycleConfigAppType".into(),
                },
                register_interface::ResultField {
                    name: "studioLifecycleConfigContent".into(),
                },
                register_interface::ResultField {
                    name: "studioLifecycleConfigName".into(),
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
        StudioLifecycleConfigResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            studio_lifecycle_config_app_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("studioLifecycleConfigAppType").unwrap(),
            ),
            studio_lifecycle_config_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("studioLifecycleConfigContent").unwrap(),
            ),
            studio_lifecycle_config_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("studioLifecycleConfigName").unwrap(),
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
