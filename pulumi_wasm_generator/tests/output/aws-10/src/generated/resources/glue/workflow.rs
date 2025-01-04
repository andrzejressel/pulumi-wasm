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
pub mod workflow {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkflowArgs {
        /// A map of default run properties for this workflow. These properties are passed to all jobs associated to the workflow.
        #[builder(into, default)]
        pub default_run_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Description of the workflow.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Prevents exceeding the maximum number of concurrent runs of any of the component jobs. If you leave this parameter blank, there is no limit to the number of concurrent workflow runs.
        #[builder(into, default)]
        pub max_concurrent_runs: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name you assign to this workflow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkflowResult {
        /// Amazon Resource Name (ARN) of Glue Workflow
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A map of default run properties for this workflow. These properties are passed to all jobs associated to the workflow.
        pub default_run_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Description of the workflow.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Prevents exceeding the maximum number of concurrent runs of any of the component jobs. If you leave this parameter blank, there is no limit to the number of concurrent workflow runs.
        pub max_concurrent_runs: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name you assign to this workflow.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: WorkflowArgs) -> WorkflowResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_run_properties_binding = args.default_run_properties.get_inner();
        let description_binding = args.description.get_inner();
        let max_concurrent_runs_binding = args.max_concurrent_runs.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/workflow:Workflow".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultRunProperties".into(),
                    value: &default_run_properties_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "maxConcurrentRuns".into(),
                    value: &max_concurrent_runs_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "defaultRunProperties".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "maxConcurrentRuns".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        WorkflowResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            default_run_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRunProperties").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            max_concurrent_runs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxConcurrentRuns").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
