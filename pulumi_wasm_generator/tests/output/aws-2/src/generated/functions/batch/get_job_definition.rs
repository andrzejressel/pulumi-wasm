pub mod get_job_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetJobDefinitionArgs {
        /// ARN of the Job Definition. Do not begin the description with "An", "The", "Defines", "Indicates", or "Specifies," as these are verbose. In other words, "Indicates the amount of storage," can be rewritten as "Amount of storage," without losing any information.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The revision of the job definition.
        #[builder(into, default)]
        pub revision: pulumi_wasm_rust::Output<Option<i32>>,
        /// The status of the job definition.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetJobDefinitionResult {
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        pub arn_prefix: pulumi_wasm_rust::Output<String>,
        /// The orchestration type of the compute environment.
        pub container_orchestration_type: pulumi_wasm_rust::Output<String>,
        /// An object with various properties that are specific to Amazon EKS based jobs. This must not be specified for Amazon ECS based job definitions.
        pub eks_properties: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetJobDefinitionEksProperty>,
        >,
        /// The ARN
        pub id: pulumi_wasm_rust::Output<String>,
        /// The name of the volume.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// An object with various properties specific to multi-node parallel jobs. If you specify node properties for a job, it becomes a multi-node parallel job. For more information, see Multi-node Parallel Jobs in the AWS Batch User Guide. If the job definition's type parameter is container, then you must specify either containerProperties or nodeProperties.
        pub node_properties: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetJobDefinitionNodeProperty>,
        >,
        /// The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that's specified during a SubmitJob operation overrides the retry strategy defined here. If a job is terminated due to a timeout, it isn't retried.
        pub retry_strategies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetJobDefinitionRetryStrategy>,
        >,
        pub revision: pulumi_wasm_rust::Output<Option<i32>>,
        /// The scheduling priority for jobs that are submitted with this job definition. This only affects jobs in job queues with a fair share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority.
        pub scheduling_priority: pulumi_wasm_rust::Output<i32>,
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The timeout configuration for jobs that are submitted with this job definition, after which AWS Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds.
        pub timeouts: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetJobDefinitionTimeout>,
        >,
        /// The type of resource to assign to a container. The supported resources include `GPU`, `MEMORY`, and `VCPU`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetJobDefinitionArgs) -> GetJobDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let name_binding = args.name.get_inner();
        let revision_binding = args.revision.get_inner();
        let status_binding = args.status.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:batch/getJobDefinition:getJobDefinition".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "revision".into(),
                    value: &revision_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "arnPrefix".into(),
                },
                register_interface::ResultField {
                    name: "containerOrchestrationType".into(),
                },
                register_interface::ResultField {
                    name: "eksProperties".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeProperties".into(),
                },
                register_interface::ResultField {
                    name: "retryStrategies".into(),
                },
                register_interface::ResultField {
                    name: "revision".into(),
                },
                register_interface::ResultField {
                    name: "schedulingPriority".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetJobDefinitionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            arn_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arnPrefix").unwrap(),
            ),
            container_orchestration_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerOrchestrationType").unwrap(),
            ),
            eks_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eksProperties").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeProperties").unwrap(),
            ),
            retry_strategies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retryStrategies").unwrap(),
            ),
            revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revision").unwrap(),
            ),
            scheduling_priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedulingPriority").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
