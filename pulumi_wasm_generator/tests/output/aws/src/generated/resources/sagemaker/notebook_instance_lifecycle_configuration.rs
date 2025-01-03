/// Provides a lifecycle configuration for SageMaker Notebook Instances.
///
/// ## Example Usage
///
/// Usage:
///
/// ```yaml
/// resources:
///   lc:
///     type: aws:sagemaker:NotebookInstanceLifecycleConfiguration
///     properties:
///       name: foo
///       onCreate:
///         fn::invoke:
///           function: std:base64encode
///           arguments:
///             input: echo foo
///           return: result
///       onStart:
///         fn::invoke:
///           function: std:base64encode
///           arguments:
///             input: echo bar
///           return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import models using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/notebookInstanceLifecycleConfiguration:NotebookInstanceLifecycleConfiguration lc foo
/// ```
pub mod notebook_instance_lifecycle_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotebookInstanceLifecycleConfigurationArgs {
        /// The name of the lifecycle configuration (must be unique). If omitted, this provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A shell script (base64-encoded) that runs only once when the SageMaker Notebook Instance is created.
        #[builder(into, default)]
        pub on_create: pulumi_wasm_rust::Output<Option<String>>,
        /// A shell script (base64-encoded) that runs every time the SageMaker Notebook Instance is started including the time it's created.
        #[builder(into, default)]
        pub on_start: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NotebookInstanceLifecycleConfigurationResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this lifecycle configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the lifecycle configuration (must be unique). If omitted, this provider will assign a random, unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A shell script (base64-encoded) that runs only once when the SageMaker Notebook Instance is created.
        pub on_create: pulumi_wasm_rust::Output<Option<String>>,
        /// A shell script (base64-encoded) that runs every time the SageMaker Notebook Instance is started including the time it's created.
        pub on_start: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NotebookInstanceLifecycleConfigurationArgs,
    ) -> NotebookInstanceLifecycleConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let on_create_binding = args.on_create.get_inner();
        let on_start_binding = args.on_start.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/notebookInstanceLifecycleConfiguration:NotebookInstanceLifecycleConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "onCreate".into(),
                    value: &on_create_binding,
                },
                register_interface::ObjectField {
                    name: "onStart".into(),
                    value: &on_start_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "onCreate".into(),
                },
                register_interface::ResultField {
                    name: "onStart".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NotebookInstanceLifecycleConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            on_create: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onCreate").unwrap(),
            ),
            on_start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onStart").unwrap(),
            ),
        }
    }
}
