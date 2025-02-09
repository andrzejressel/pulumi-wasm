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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod autoscaling_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutoscalingPolicyArgs {
        /// Basic algorithm for autoscaling.
        /// Structure is documented below.
        #[builder(into, default)]
        pub basic_algorithm: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::AutoscalingPolicyBasicAlgorithm>,
        >,
        /// The  location where the autoscaling policy should reside.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),
        /// and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between
        /// 3 and 50 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Describes how the autoscaler will operate for secondary workers.
        /// Structure is documented below.
        #[builder(into, default)]
        pub secondary_worker_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::AutoscalingPolicySecondaryWorkerConfig>,
        >,
        /// Describes how the autoscaler will operate for primary workers.
        /// Structure is documented below.
        #[builder(into, default)]
        pub worker_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::AutoscalingPolicyWorkerConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct AutoscalingPolicyResult {
        /// Basic algorithm for autoscaling.
        /// Structure is documented below.
        pub basic_algorithm: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::AutoscalingPolicyBasicAlgorithm>,
        >,
        /// The  location where the autoscaling policy should reside.
        /// The default value is `global`.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The "resource name" of the autoscaling policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),
        /// and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between
        /// 3 and 50 characters.
        ///
        ///
        /// - - -
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Describes how the autoscaler will operate for secondary workers.
        /// Structure is documented below.
        pub secondary_worker_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::AutoscalingPolicySecondaryWorkerConfig>,
        >,
        /// Describes how the autoscaler will operate for primary workers.
        /// Structure is documented below.
        pub worker_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::AutoscalingPolicyWorkerConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutoscalingPolicyArgs,
    ) -> AutoscalingPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let basic_algorithm_binding = args.basic_algorithm.get_output(context);
        let location_binding = args.location.get_output(context);
        let policy_id_binding = args.policy_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let secondary_worker_config_binding = args
            .secondary_worker_config
            .get_output(context);
        let worker_config_binding = args.worker_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataproc/autoscalingPolicy:AutoscalingPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "basicAlgorithm".into(),
                    value: basic_algorithm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondaryWorkerConfig".into(),
                    value: secondary_worker_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workerConfig".into(),
                    value: worker_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutoscalingPolicyResult {
            basic_algorithm: o.get_field("basicAlgorithm"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            policy_id: o.get_field("policyId"),
            project: o.get_field("project"),
            secondary_worker_config: o.get_field("secondaryWorkerConfig"),
            worker_config: o.get_field("workerConfig"),
        }
    }
}
