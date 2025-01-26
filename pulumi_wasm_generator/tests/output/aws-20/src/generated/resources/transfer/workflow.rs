/// Provides a AWS Transfer Workflow resource.
///
/// ## Example Usage
///
/// ### Basic single step example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workflow::create(
///         "example",
///         WorkflowArgs::builder()
///             .steps(
///                 vec![
///                     WorkflowStep::builder()
///                     .deleteStepDetails(WorkflowStepDeleteStepDetails::builder()
///                     .name("example").sourceFileLocation("$${original.file}")
///                     .build_struct()). type ("DELETE").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Multistep example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workflow::create(
///         "example",
///         WorkflowArgs::builder()
///             .steps(
///                 vec![
///                     WorkflowStep::builder()
///                     .customStepDetails(WorkflowStepCustomStepDetails::builder()
///                     .name("example").sourceFileLocation("$${original.file}")
///                     .target("${exampleAwsLambdaFunction.arn}").timeoutSeconds(60)
///                     .build_struct()). type ("CUSTOM").build_struct(),
///                     WorkflowStep::builder()
///                     .tagStepDetails(WorkflowStepTagStepDetails::builder().name("example")
///                     .sourceFileLocation("$${original.file}")
///                     .tags(vec![WorkflowStepTagStepDetailsTag::builder().key("Name")
///                     .value("Hello World").build_struct(),]).build_struct()). type ("TAG")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transfer Workflows using the `worflow_id`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/workflow:Workflow example example
/// ```
pub mod workflow {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkflowArgs {
        /// A textual description for the workflow.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the steps (actions) to take if errors are encountered during execution of the workflow. See Workflow Steps below.
        #[builder(into, default)]
        pub on_exception_steps: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::transfer::WorkflowOnExceptionStep>>,
        >,
        /// Specifies the details for the steps that are in the specified workflow. See Workflow Steps below.
        #[builder(into)]
        pub steps: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::transfer::WorkflowStep>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkflowResult {
        /// The Workflow ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A textual description for the workflow.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the steps (actions) to take if errors are encountered during execution of the workflow. See Workflow Steps below.
        pub on_exception_steps: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::transfer::WorkflowOnExceptionStep>>,
        >,
        /// Specifies the details for the steps that are in the specified workflow. See Workflow Steps below.
        pub steps: pulumi_wasm_rust::Output<
            Vec<super::super::types::transfer::WorkflowStep>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WorkflowArgs,
    ) -> WorkflowResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let on_exception_steps_binding = args
            .on_exception_steps
            .get_output(context)
            .get_inner();
        let steps_binding = args.steps.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transfer/workflow:Workflow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "onExceptionSteps".into(),
                    value: &on_exception_steps_binding,
                },
                register_interface::ObjectField {
                    name: "steps".into(),
                    value: &steps_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "onExceptionSteps".into(),
                },
                register_interface::ResultField {
                    name: "steps".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkflowResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            on_exception_steps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onExceptionSteps").unwrap(),
            ),
            steps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("steps").unwrap(),
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
