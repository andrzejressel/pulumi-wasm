/// Provides a Glue Workflow resource.
/// The workflow graph (DAG) can be build using the `aws.glue.Trigger` resource.
/// See the example below for creating a graph with four nodes (two triggers and two jobs).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:glue:Workflow
///     properties:
///       name: example
///   example-start:
///     type: aws:glue:Trigger
///     properties:
///       name: trigger-start
///       type: ON_DEMAND
///       workflowName: ${example.name}
///       actions:
///         - jobName: example-job
///   example-inner:
///     type: aws:glue:Trigger
///     properties:
///       name: trigger-inner
///       type: CONDITIONAL
///       workflowName: ${example.name}
///       predicate:
///         conditions:
///           - jobName: example-job
///             state: SUCCEEDED
///       actions:
///         - jobName: another-example-job
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Workflows using `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/workflow:Workflow MyWorkflow MyWorkflow
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workflow {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkflowArgs {
        /// A map of default run properties for this workflow. These properties are passed to all jobs associated to the workflow.
        #[builder(into, default)]
        pub default_run_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Description of the workflow.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Prevents exceeding the maximum number of concurrent runs of any of the component jobs. If you leave this parameter blank, there is no limit to the number of concurrent workflow runs.
        #[builder(into, default)]
        pub max_concurrent_runs: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name you assign to this workflow.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkflowResult {
        /// Amazon Resource Name (ARN) of Glue Workflow
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A map of default run properties for this workflow. These properties are passed to all jobs associated to the workflow.
        pub default_run_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Description of the workflow.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Prevents exceeding the maximum number of concurrent runs of any of the component jobs. If you leave this parameter blank, there is no limit to the number of concurrent workflow runs.
        pub max_concurrent_runs: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name you assign to this workflow.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: WorkflowArgs,
    ) -> WorkflowResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_run_properties_binding = args
            .default_run_properties
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let max_concurrent_runs_binding = args.max_concurrent_runs.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/workflow:Workflow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRunProperties".into(),
                    value: default_run_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxConcurrentRuns".into(),
                    value: max_concurrent_runs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkflowResult {
            arn: o.get_field("arn"),
            default_run_properties: o.get_field("defaultRunProperties"),
            description: o.get_field("description"),
            max_concurrent_runs: o.get_field("maxConcurrentRuns"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
