/// Resource for managing an AWS CodeGuru Profiler Profiling Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod profiling_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfilingGroupArgs {
        /// Specifies whether profiling is enabled or disabled for the created profiling. See Agent Orchestration Config for more details.
        #[builder(into, default)]
        pub agent_orchestration_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::codeguruprofiler::ProfilingGroupAgentOrchestrationConfig,
            >,
        >,
        /// Compute platform of the profiling group.
        #[builder(into, default)]
        pub compute_platform: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the profiling group.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfilingGroupResult {
        /// Specifies whether profiling is enabled or disabled for the created profiling. See Agent Orchestration Config for more details.
        pub agent_orchestration_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::codeguruprofiler::ProfilingGroupAgentOrchestrationConfig,
            >,
        >,
        /// ARN of the profiling group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Compute platform of the profiling group.
        pub compute_platform: pulumi_wasm_rust::Output<String>,
        /// Name of the profiling group.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProfilingGroupArgs,
    ) -> ProfilingGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_orchestration_config_binding = args
            .agent_orchestration_config
            .get_output(context)
            .get_inner();
        let compute_platform_binding = args
            .compute_platform
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codeguruprofiler/profilingGroup:ProfilingGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentOrchestrationConfig".into(),
                    value: &agent_orchestration_config_binding,
                },
                register_interface::ObjectField {
                    name: "computePlatform".into(),
                    value: &compute_platform_binding,
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
                    name: "agentOrchestrationConfig".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "computePlatform".into(),
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProfilingGroupResult {
            agent_orchestration_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentOrchestrationConfig").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            compute_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computePlatform").unwrap(),
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
