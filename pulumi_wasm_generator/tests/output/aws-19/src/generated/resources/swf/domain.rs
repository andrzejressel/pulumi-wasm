/// Provides an SWF Domain resource.
///
/// ## Example Usage
///
/// To register a basic SWF domain:
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:swf:Domain
///     properties:
///       name: foo
///       description: SWF Domain
///       workflowExecutionRetentionPeriodInDays: 30
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SWF Domains using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:swf/domain:Domain foo test-domain
/// ```
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// The domain description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the domain. If omitted, this provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Length of time that SWF will continue to retain information about the workflow execution after the workflow execution is complete, must be between 0 and 90 days.
        #[builder(into)]
        pub workflow_execution_retention_period_in_days: pulumi_wasm_rust::Output<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// Amazon Resource Name (ARN)
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The domain description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the domain. If omitted, this provider will assign a random, unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Length of time that SWF will continue to retain information about the workflow execution after the workflow execution is complete, must be between 0 and 90 days.
        pub workflow_execution_retention_period_in_days: pulumi_wasm_rust::Output<
            String,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainArgs) -> DomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let tags_binding = args.tags.get_inner();
        let workflow_execution_retention_period_in_days_binding = args
            .workflow_execution_retention_period_in_days
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:swf/domain:Domain".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workflowExecutionRetentionPeriodInDays".into(),
                    value: &workflow_execution_retention_period_in_days_binding,
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "workflowExecutionRetentionPeriodInDays".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            workflow_execution_retention_period_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workflowExecutionRetentionPeriodInDays").unwrap(),
            ),
        }
    }
}
