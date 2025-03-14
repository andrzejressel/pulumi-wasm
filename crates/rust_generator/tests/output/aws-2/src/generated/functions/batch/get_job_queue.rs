#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_job_queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetJobQueueArgs {
        /// Name of the job queue.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetJobQueueResult {
        /// ARN of the job queue.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The compute environments that are attached to the job queue and the order in
        /// which job placement is preferred. Compute environments are selected for job placement in ascending order.
        /// * `compute_environment_order.#.order` - The order of the compute environment.
        /// * `compute_environment_order.#.compute_environment` - The ARN of the compute environment.
        pub compute_environment_orders: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetJobQueueComputeEnvironmentOrder>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies an action that AWS Batch will take after the job has remained at the head of the queue in the specified state for longer than the specified time.
        /// * `job_state_time_limit_action.#.action` - The action to take when a job is at the head of the job queue in the specified state for the specified period of time.
        /// * `job_state_time_limit_action.#.max_time_seconds` - The approximate amount of time, in seconds, that must pass with the job in the specified state before the action is taken.
        /// * `job_state_time_limit_action.#.reason` - The reason to log for the action being taken.
        /// * `job_state_time_limit_action.#.state` - The state of the job needed to trigger the action.
        pub job_state_time_limit_actions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetJobQueueJobStateTimeLimitAction>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Priority of the job queue. Job queues with a higher priority are evaluated first when
        /// associated with the same compute environment.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// The ARN of the fair share scheduling policy. If this attribute has a value, the job queue uses a fair share scheduling policy. If this attribute does not have a value, the job queue uses a first in, first out (FIFO) scheduling policy.
        pub scheduling_policy_arn: pulumi_gestalt_rust::Output<String>,
        /// Describes the ability of the queue to accept new jobs (for example, `ENABLED` or `DISABLED`).
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Current status of the job queue (for example, `CREATING` or `VALID`).
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Short, human-readable string to provide additional details about the current status
        /// of the job queue.
        pub status_reason: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetJobQueueArgs,
    ) -> GetJobQueueResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:batch/getJobQueue:getJobQueue".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetJobQueueResult {
            arn: o.get_field("arn"),
            compute_environment_orders: o.get_field("computeEnvironmentOrders"),
            id: o.get_field("id"),
            job_state_time_limit_actions: o.get_field("jobStateTimeLimitActions"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            scheduling_policy_arn: o.get_field("schedulingPolicyArn"),
            state: o.get_field("state"),
            status: o.get_field("status"),
            status_reason: o.get_field("statusReason"),
            tags: o.get_field("tags"),
        }
    }
}
