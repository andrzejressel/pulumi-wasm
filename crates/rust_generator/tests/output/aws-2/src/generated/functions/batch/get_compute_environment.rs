#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_compute_environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetComputeEnvironmentArgs {
        /// Name of the Batch Compute Environment
        #[builder(into)]
        pub compute_environment_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetComputeEnvironmentResult {
        /// ARN of the compute environment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub compute_environment_name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the underlying Amazon ECS cluster used by the compute environment.
        pub ecs_cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf.
        pub service_role: pulumi_gestalt_rust::Output<String>,
        /// State of the compute environment (for example, `ENABLED` or `DISABLED`). If the state is `ENABLED`, then the compute environment accepts jobs from a queue and can scale out automatically based on queues.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Current status of the compute environment (for example, `CREATING` or `VALID`).
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Short, human-readable string to provide additional details about the current status of the compute environment.
        pub status_reason: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of the compute environment (for example, `MANAGED` or `UNMANAGED`).
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Specifies the infrastructure update policy for the compute environment.
        pub update_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetComputeEnvironmentUpdatePolicy>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetComputeEnvironmentArgs,
    ) -> GetComputeEnvironmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let compute_environment_name_binding = args
            .compute_environment_name
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:batch/getComputeEnvironment:getComputeEnvironment".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeEnvironmentName".into(),
                    value: &compute_environment_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetComputeEnvironmentResult {
            arn: o.get_field("arn"),
            compute_environment_name: o.get_field("computeEnvironmentName"),
            ecs_cluster_arn: o.get_field("ecsClusterArn"),
            id: o.get_field("id"),
            service_role: o.get_field("serviceRole"),
            state: o.get_field("state"),
            status: o.get_field("status"),
            status_reason: o.get_field("statusReason"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            update_policies: o.get_field("updatePolicies"),
        }
    }
}
