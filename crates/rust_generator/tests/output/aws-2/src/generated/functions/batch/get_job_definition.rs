#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_job_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetJobDefinitionArgs {
        /// ARN of the Job Definition. Do not begin the description with "An", "The", "Defines", "Indicates", or "Specifies," as these are verbose. In other words, "Indicates the amount of storage," can be rewritten as "Amount of storage," without losing any information.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The revision of the job definition.
        #[builder(into, default)]
        pub revision: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The status of the job definition.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetJobDefinitionResult {
        pub arn: pulumi_gestalt_rust::Output<Option<String>>,
        pub arn_prefix: pulumi_gestalt_rust::Output<String>,
        /// The orchestration type of the compute environment.
        pub container_orchestration_type: pulumi_gestalt_rust::Output<String>,
        /// An object with various properties that are specific to Amazon EKS based jobs. This must not be specified for Amazon ECS based job definitions.
        pub eks_properties: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetJobDefinitionEksProperty>,
        >,
        /// The ARN
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the volume.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// An object with various properties specific to multi-node parallel jobs. If you specify node properties for a job, it becomes a multi-node parallel job. For more information, see Multi-node Parallel Jobs in the AWS Batch User Guide. If the job definition's type parameter is container, then you must specify either containerProperties or nodeProperties.
        pub node_properties: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetJobDefinitionNodeProperty>,
        >,
        /// The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that's specified during a SubmitJob operation overrides the retry strategy defined here. If a job is terminated due to a timeout, it isn't retried.
        pub retry_strategies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetJobDefinitionRetryStrategy>,
        >,
        pub revision: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The scheduling priority for jobs that are submitted with this job definition. This only affects jobs in job queues with a fair share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority.
        pub scheduling_priority: pulumi_gestalt_rust::Output<i32>,
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The timeout configuration for jobs that are submitted with this job definition, after which AWS Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds.
        pub timeouts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetJobDefinitionTimeout>,
        >,
        /// The type of resource to assign to a container. The supported resources include `GPU`, `MEMORY`, and `VCPU`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetJobDefinitionArgs,
    ) -> GetJobDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let revision_binding = args.revision.get_output(context);
        let status_binding = args.status.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:batch/getJobDefinition:getJobDefinition".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "revision".into(),
                    value: &revision_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetJobDefinitionResult {
            arn: o.get_field("arn"),
            arn_prefix: o.get_field("arnPrefix"),
            container_orchestration_type: o.get_field("containerOrchestrationType"),
            eks_properties: o.get_field("eksProperties"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            node_properties: o.get_field("nodeProperties"),
            retry_strategies: o.get_field("retryStrategies"),
            revision: o.get_field("revision"),
            scheduling_priority: o.get_field("schedulingPriority"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            timeouts: o.get_field("timeouts"),
            type_: o.get_field("type"),
        }
    }
}
