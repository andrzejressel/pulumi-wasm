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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod notebook_instance_lifecycle_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotebookInstanceLifecycleConfigurationArgs {
        /// The name of the lifecycle configuration (must be unique). If omitted, this provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A shell script (base64-encoded) that runs only once when the SageMaker Notebook Instance is created.
        #[builder(into, default)]
        pub on_create: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A shell script (base64-encoded) that runs every time the SageMaker Notebook Instance is started including the time it's created.
        #[builder(into, default)]
        pub on_start: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NotebookInstanceLifecycleConfigurationResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this lifecycle configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the lifecycle configuration (must be unique). If omitted, this provider will assign a random, unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A shell script (base64-encoded) that runs only once when the SageMaker Notebook Instance is created.
        pub on_create: pulumi_gestalt_rust::Output<Option<String>>,
        /// A shell script (base64-encoded) that runs every time the SageMaker Notebook Instance is started including the time it's created.
        pub on_start: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotebookInstanceLifecycleConfigurationArgs,
    ) -> NotebookInstanceLifecycleConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let on_create_binding = args.on_create.get_output(context);
        let on_start_binding = args.on_start.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/notebookInstanceLifecycleConfiguration:NotebookInstanceLifecycleConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onCreate".into(),
                    value: on_create_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onStart".into(),
                    value: on_start_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NotebookInstanceLifecycleConfigurationResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            on_create: o.get_field("onCreate"),
            on_start: o.get_field("onStart"),
        }
    }
}
