pub mod get_profiling_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProfilingGroupArgs {
        /// The name of the profiling group.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetProfilingGroupResult {
        /// Profiling Group agent orchestration config
        pub agent_orchestration_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::codeguruprofiler::GetProfilingGroupAgentOrchestrationConfig,
            >,
        >,
        /// ARN of the Profiling Group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The compute platform of the profiling group.
        pub compute_platform: pulumi_wasm_rust::Output<String>,
        /// Timestamp when Profiling Group was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The status of the Profiling Group.
        pub profiling_statuses: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::codeguruprofiler::GetProfilingGroupProfilingStatus,
            >,
        >,
        /// Mapping of Key-Value tags for the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp when Profiling Group was updated.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetProfilingGroupArgs) -> GetProfilingGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codeguruprofiler/getProfilingGroup:getProfilingGroup".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "agentOrchestrationConfigs".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "computePlatform".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "profilingStatuses".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProfilingGroupResult {
            agent_orchestration_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentOrchestrationConfigs").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            compute_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computePlatform").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            profiling_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profilingStatuses").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}
