/// Provides a Batch Job Queue resource.
///
/// ## Example Usage
///
/// ### Basic Job Queue
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testQueue = job_queue::create(
///         "testQueue",
///         JobQueueArgs::builder()
///             .compute_environment_orders(
///                 vec![
///                     JobQueueComputeEnvironmentOrder::builder()
///                     .computeEnvironment("${testEnvironment1.arn}").order(1)
///                     .build_struct(), JobQueueComputeEnvironmentOrder::builder()
///                     .computeEnvironment("${testEnvironment2.arn}").order(2)
///                     .build_struct(),
///                 ],
///             )
///             .name("tf-test-batch-job-queue")
///             .priority(1)
///             .state("ENABLED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Job Queue with a fair share scheduling policy
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = scheduling_policy::create(
///         "example",
///         SchedulingPolicyArgs::builder()
///             .fair_share_policy(
///                 SchedulingPolicyFairSharePolicy::builder()
///                     .computeReservation(1)
///                     .shareDecaySeconds(3600)
///                     .shareDistributions(
///                         vec![
///                             SchedulingPolicyFairSharePolicyShareDistribution::builder()
///                             .shareIdentifier("A1*").weightFactor(0.1).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
///     let exampleJobQueue = job_queue::create(
///         "exampleJobQueue",
///         JobQueueArgs::builder()
///             .compute_environment_orders(
///                 vec![
///                     JobQueueComputeEnvironmentOrder::builder()
///                     .computeEnvironment("${testEnvironment1.arn}").order(1)
///                     .build_struct(), JobQueueComputeEnvironmentOrder::builder()
///                     .computeEnvironment("${testEnvironment2.arn}").order(2)
///                     .build_struct(),
///                 ],
///             )
///             .name("tf-test-batch-job-queue")
///             .priority(1)
///             .scheduling_policy_arn("${example.arn}")
///             .state("ENABLED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Batch Job Queue using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:batch/jobQueue:JobQueue test_queue arn:aws:batch:us-east-1:123456789012:job-queue/sample
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job_queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobQueueArgs {
        /// The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment runs a specific job. Compute environments must be in the VALID state before you can associate them with a job queue. You can associate up to three compute environments with a job queue.
        #[builder(into, default)]
        pub compute_environment_orders: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::JobQueueComputeEnvironmentOrder>>,
        >,
        /// (Optional) This parameter is deprecated, please use `compute_environment_order` instead. List of compute environment ARNs mapped to a job queue. The position of the compute environments in the list will dictate the order. When importing a AWS Batch Job Queue, the parameter `compute_environments` will always be used over `compute_environment_order`. Please adjust your HCL accordingly.
        #[builder(into, default)]
        pub compute_environments: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The set of job state time limit actions mapped to a job queue. Specifies an action that AWS Batch will take after the job has remained at the head of the queue in the specified state for longer than the specified time.
        #[builder(into, default)]
        pub job_state_time_limit_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::batch::JobQueueJobStateTimeLimitAction>>,
        >,
        /// Specifies the name of the job queue.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The priority of the job queue. Job queues with a higher priority
        /// are evaluated first when associated with the same compute environment.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ARN of the fair share scheduling policy. If this parameter is specified, the job queue uses a fair share scheduling policy. If this parameter isn't specified, the job queue uses a first in, first out (FIFO) scheduling policy. After a job queue is created, you can replace but can't remove the fair share scheduling policy.
        #[builder(into, default)]
        pub scheduling_policy_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The state of the job queue. Must be one of: `ENABLED` or `DISABLED`
        #[builder(into)]
        pub state: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::JobQueueTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct JobQueueResult {
        /// The Amazon Resource Name of the job queue.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment runs a specific job. Compute environments must be in the VALID state before you can associate them with a job queue. You can associate up to three compute environments with a job queue.
        pub compute_environment_orders: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::JobQueueComputeEnvironmentOrder>>,
        >,
        /// (Optional) This parameter is deprecated, please use `compute_environment_order` instead. List of compute environment ARNs mapped to a job queue. The position of the compute environments in the list will dictate the order. When importing a AWS Batch Job Queue, the parameter `compute_environments` will always be used over `compute_environment_order`. Please adjust your HCL accordingly.
        pub compute_environments: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The set of job state time limit actions mapped to a job queue. Specifies an action that AWS Batch will take after the job has remained at the head of the queue in the specified state for longer than the specified time.
        pub job_state_time_limit_actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::batch::JobQueueJobStateTimeLimitAction>>,
        >,
        /// Specifies the name of the job queue.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The priority of the job queue. Job queues with a higher priority
        /// are evaluated first when associated with the same compute environment.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// The ARN of the fair share scheduling policy. If this parameter is specified, the job queue uses a fair share scheduling policy. If this parameter isn't specified, the job queue uses a first in, first out (FIFO) scheduling policy. After a job queue is created, you can replace but can't remove the fair share scheduling policy.
        pub scheduling_policy_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The state of the job queue. Must be one of: `ENABLED` or `DISABLED`
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::JobQueueTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: JobQueueArgs,
    ) -> JobQueueResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let compute_environment_orders_binding = args
            .compute_environment_orders
            .get_output(context);
        let compute_environments_binding = args.compute_environments.get_output(context);
        let job_state_time_limit_actions_binding = args
            .job_state_time_limit_actions
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let scheduling_policy_arn_binding = args
            .scheduling_policy_arn
            .get_output(context);
        let state_binding = args.state.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:batch/jobQueue:JobQueue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeEnvironmentOrders".into(),
                    value: compute_environment_orders_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeEnvironments".into(),
                    value: compute_environments_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jobStateTimeLimitActions".into(),
                    value: job_state_time_limit_actions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedulingPolicyArn".into(),
                    value: scheduling_policy_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        JobQueueResult {
            arn: o.get_field("arn"),
            compute_environment_orders: o.get_field("computeEnvironmentOrders"),
            compute_environments: o.get_field("computeEnvironments"),
            job_state_time_limit_actions: o.get_field("jobStateTimeLimitActions"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            scheduling_policy_arn: o.get_field("schedulingPolicyArn"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
