#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_profiling_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProfilingGroupArgs {
        /// The name of the profiling group.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetProfilingGroupResult {
        /// Profiling Group agent orchestration config
        pub agent_orchestration_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::codeguruprofiler::GetProfilingGroupAgentOrchestrationConfig,
            >,
        >,
        /// ARN of the Profiling Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The compute platform of the profiling group.
        pub compute_platform: pulumi_gestalt_rust::Output<String>,
        /// Timestamp when Profiling Group was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The status of the Profiling Group.
        pub profiling_statuses: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::codeguruprofiler::GetProfilingGroupProfilingStatus,
            >,
        >,
        /// Mapping of Key-Value tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp when Profiling Group was updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetProfilingGroupArgs,
    ) -> GetProfilingGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:codeguruprofiler/getProfilingGroup:getProfilingGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetProfilingGroupResult {
            agent_orchestration_configs: o.get_field("agentOrchestrationConfigs"),
            arn: o.get_field("arn"),
            compute_platform: o.get_field("computePlatform"),
            created_at: o.get_field("createdAt"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            profiling_statuses: o.get_field("profilingStatuses"),
            tags: o.get_field("tags"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
