/// Describes an autoscaling policy for Dataproc cluster autoscaler.
///
///
///
/// ## Example Usage
///
/// ### Dataproc Autoscaling Policy
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let asp = autoscaling_policy::create(
///         "asp",
///         AutoscalingPolicyArgs::builder()
///             .basic_algorithm(
///                 AutoscalingPolicyBasicAlgorithm::builder()
///                     .yarnConfig(
///                         AutoscalingPolicyBasicAlgorithmYarnConfig::builder()
///                             .gracefulDecommissionTimeout("30s")
///                             .scaleDownFactor(0.5)
///                             .scaleUpFactor(0.5)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .policy_id("dataproc-policy")
///             .worker_config(
///                 AutoscalingPolicyWorkerConfig::builder().maxInstances(3).build_struct(),
///             )
///             .build_struct(),
///     );
///     let basic = cluster::create(
///         "basic",
///         ClusterArgs::builder()
///             .cluster_config(
///                 ClusterClusterConfig::builder()
///                     .autoscalingConfig(
///                         ClusterClusterConfigAutoscalingConfig::builder()
///                             .policyUri("${asp.name}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("dataproc-policy")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AutoscalingPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/autoscalingPolicies/{{policy_id}}`
///
/// * `{{project}}/{{location}}/{{policy_id}}`
///
/// * `{{location}}/{{policy_id}}`
///
/// When using the `pulumi import` command, AutoscalingPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataproc/autoscalingPolicy:AutoscalingPolicy default projects/{{project}}/locations/{{location}}/autoscalingPolicies/{{policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/autoscalingPolicy:AutoscalingPolicy default {{project}}/{{location}}/{{policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/autoscalingPolicy:AutoscalingPolicy default {{location}}/{{policy_id}}
/// ```
///
pub mod autoscaling_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutoscalingPolicyArgs {
        /// Basic algorithm for autoscaling.
        /// Structure is documented below.
        #[builder(into, default)]
        pub basic_algorithm: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::AutoscalingPolicyBasicAlgorithm>,
        >,
        /// The  location where the autoscaling policy should reside.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),
        /// and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between
        /// 3 and 50 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub policy_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Describes how the autoscaler will operate for secondary workers.
        /// Structure is documented below.
        #[builder(into, default)]
        pub secondary_worker_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::AutoscalingPolicySecondaryWorkerConfig>,
        >,
        /// Describes how the autoscaler will operate for primary workers.
        /// Structure is documented below.
        #[builder(into, default)]
        pub worker_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::AutoscalingPolicyWorkerConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct AutoscalingPolicyResult {
        /// Basic algorithm for autoscaling.
        /// Structure is documented below.
        pub basic_algorithm: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::AutoscalingPolicyBasicAlgorithm>,
        >,
        /// The  location where the autoscaling policy should reside.
        /// The default value is `global`.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The "resource name" of the autoscaling policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),
        /// and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between
        /// 3 and 50 characters.
        ///
        ///
        /// - - -
        pub policy_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Describes how the autoscaler will operate for secondary workers.
        /// Structure is documented below.
        pub secondary_worker_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::AutoscalingPolicySecondaryWorkerConfig>,
        >,
        /// Describes how the autoscaler will operate for primary workers.
        /// Structure is documented below.
        pub worker_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::AutoscalingPolicyWorkerConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AutoscalingPolicyArgs,
    ) -> AutoscalingPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let basic_algorithm_binding = args
            .basic_algorithm
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let policy_id_binding = args.policy_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let secondary_worker_config_binding = args
            .secondary_worker_config
            .get_output(context)
            .get_inner();
        let worker_config_binding = args.worker_config.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/autoscalingPolicy:AutoscalingPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "basicAlgorithm".into(),
                    value: &basic_algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryWorkerConfig".into(),
                    value: &secondary_worker_config_binding,
                },
                register_interface::ObjectField {
                    name: "workerConfig".into(),
                    value: &worker_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "basicAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policyId".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "secondaryWorkerConfig".into(),
                },
                register_interface::ResultField {
                    name: "workerConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AutoscalingPolicyResult {
            basic_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("basicAlgorithm").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyId").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            secondary_worker_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryWorkerConfig").unwrap(),
            ),
            worker_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerConfig").unwrap(),
            ),
        }
    }
}
