/// Provides a SageMaker Human Task UI resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sagemaker:HumanTaskUI
///     properties:
///       humanTaskUiName: example
///       uiTemplate:
///         content:
///           fn::invoke:
///             Function: std:file
///             Arguments:
///               input: sagemaker-human-task-ui-template.html
///             Return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Human Task UIs using the `human_task_ui_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/humanTaskUI:HumanTaskUI example example
/// ```
pub mod human_task_ui {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HumanTaskUIArgs {
        /// The name of the Human Task UI.
        #[builder(into)]
        pub human_task_ui_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Liquid template for the worker user interface. See UI Template below.
        #[builder(into)]
        pub ui_template: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::HumanTaskUiUiTemplate,
        >,
    }
    #[allow(dead_code)]
    pub struct HumanTaskUIResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Human Task UI.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Human Task UI.
        pub human_task_ui_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Liquid template for the worker user interface. See UI Template below.
        pub ui_template: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::HumanTaskUiUiTemplate,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HumanTaskUIArgs) -> HumanTaskUIResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let human_task_ui_name_binding = args.human_task_ui_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let ui_template_binding = args.ui_template.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/humanTaskUI:HumanTaskUI".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "humanTaskUiName".into(),
                    value: &human_task_ui_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "uiTemplate".into(),
                    value: &ui_template_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "humanTaskUiName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "uiTemplate".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HumanTaskUIResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            human_task_ui_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("humanTaskUiName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            ui_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uiTemplate").unwrap(),
            ),
        }
    }
}