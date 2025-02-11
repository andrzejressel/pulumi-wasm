/// Resource for managing an AWS CodeGuru Profiler Profiling Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = profiling_group::create(
///         "example",
///         ProfilingGroupArgs::builder()
///             .agent_orchestration_config(
///                 ProfilingGroupAgentOrchestrationConfig::builder()
///                     .profilingEnabled(true)
///                     .build_struct(),
///             )
///             .compute_platform("Default")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeGuru Profiler Profiling Group using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:codeguruprofiler/profilingGroup:ProfilingGroup example profiling_group-name-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod profiling_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfilingGroupArgs {
        /// Specifies whether profiling is enabled or disabled for the created profiling. See Agent Orchestration Config for more details.
        #[builder(into, default)]
        pub agent_orchestration_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::codeguruprofiler::ProfilingGroupAgentOrchestrationConfig,
            >,
        >,
        /// Compute platform of the profiling group.
        #[builder(into, default)]
        pub compute_platform: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the profiling group.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfilingGroupResult {
        /// Specifies whether profiling is enabled or disabled for the created profiling. See Agent Orchestration Config for more details.
        pub agent_orchestration_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::codeguruprofiler::ProfilingGroupAgentOrchestrationConfig,
            >,
        >,
        /// ARN of the profiling group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Compute platform of the profiling group.
        pub compute_platform: pulumi_gestalt_rust::Output<String>,
        /// Name of the profiling group.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: ProfilingGroupArgs,
    ) -> ProfilingGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_orchestration_config_binding = args
            .agent_orchestration_config
            .get_output(context);
        let compute_platform_binding = args.compute_platform.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codeguruprofiler/profilingGroup:ProfilingGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentOrchestrationConfig".into(),
                    value: &agent_orchestration_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computePlatform".into(),
                    value: &compute_platform_binding.drop_type(),
                },
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
        let o = context.register_resource(request);
        ProfilingGroupResult {
            agent_orchestration_config: o.get_field("agentOrchestrationConfig"),
            arn: o.get_field("arn"),
            compute_platform: o.get_field("computePlatform"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
