pub mod get_job_queue {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetJobQueueArgs {
        /// Name of the job queue.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetJobQueueResult {
        /// ARN of the job queue.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The compute environments that are attached to the job queue and the order in
        /// which job placement is preferred. Compute environments are selected for job placement in ascending order.
        /// * `compute_environment_order.#.order` - The order of the compute environment.
        /// * `compute_environment_order.#.compute_environment` - The ARN of the compute environment.
        pub compute_environment_orders: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetJobQueueComputeEnvironmentOrder>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Specifies an action that AWS Batch will take after the job has remained at the head of the queue in the specified state for longer than the specified time.
        /// * `job_state_time_limit_action.#.action` - The action to take when a job is at the head of the job queue in the specified state for the specified period of time.
        /// * `job_state_time_limit_action.#.max_time_seconds` - The approximate amount of time, in seconds, that must pass with the job in the specified state before the action is taken.
        /// * `job_state_time_limit_action.#.reason` - The reason to log for the action being taken.
        /// * `job_state_time_limit_action.#.state` - The state of the job needed to trigger the action.
        pub job_state_time_limit_actions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetJobQueueJobStateTimeLimitAction>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Priority of the job queue. Job queues with a higher priority are evaluated first when
        /// associated with the same compute environment.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The ARN of the fair share scheduling policy. If this attribute has a value, the job queue uses a fair share scheduling policy. If this attribute does not have a value, the job queue uses a first in, first out (FIFO) scheduling policy.
        pub scheduling_policy_arn: pulumi_wasm_rust::Output<String>,
        /// Describes the ability of the queue to accept new jobs (for example, `ENABLED` or `DISABLED`).
        pub state: pulumi_wasm_rust::Output<String>,
        /// Current status of the job queue (for example, `CREATING` or `VALID`).
        pub status: pulumi_wasm_rust::Output<String>,
        /// Short, human-readable string to provide additional details about the current status
        /// of the job queue.
        pub status_reason: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetJobQueueArgs) -> GetJobQueueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:batch/getJobQueue:getJobQueue".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
                    name: "computeEnvironmentOrders".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "jobStateTimeLimitActions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "schedulingPolicyArn".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusReason".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetJobQueueResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            compute_environment_orders: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeEnvironmentOrders").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            job_state_time_limit_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobStateTimeLimitActions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            scheduling_policy_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedulingPolicyArn").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusReason").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
