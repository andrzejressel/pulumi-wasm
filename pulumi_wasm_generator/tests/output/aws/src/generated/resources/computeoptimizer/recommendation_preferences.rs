/// Manages AWS Compute Optimizer recommendation preferences.
///
/// ## Example Usage
///
/// ### Lookback Period Preference
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = recommendation_preferences::create(
///         "example",
///         RecommendationPreferencesArgs::builder()
///             .look_back_period("DAYS_32")
///             .resource_type("Ec2Instance")
///             .scope(
///                 RecommendationPreferencesScope::builder()
///                     .name("AccountId")
///                     .value("123456789012")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Multiple Preferences
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = recommendation_preferences::create(
///         "example",
///         RecommendationPreferencesArgs::builder()
///             .enhanced_infrastructure_metrics("Active")
///             .external_metrics_preference(
///                 RecommendationPreferencesExternalMetricsPreference::builder()
///                     .source("Datadog")
///                     .build_struct(),
///             )
///             .preferred_resources(
///                 vec![
///                     RecommendationPreferencesPreferredResource::builder()
///                     .includeLists(vec!["m5.xlarge", "r5",]).name("Ec2InstanceTypes")
///                     .build_struct(),
///                 ],
///             )
///             .resource_type("Ec2Instance")
///             .scope(
///                 RecommendationPreferencesScope::builder()
///                     .name("AccountId")
///                     .value("123456789012")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import recommendation preferences using the resource type, scope name and scope value. For example:
///
/// ```sh
/// $ pulumi import aws:computeoptimizer/recommendationPreferences:RecommendationPreferences example Ec2Instance,AccountId,123456789012
/// ```
pub mod recommendation_preferences {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecommendationPreferencesArgs {
        /// The status of the enhanced infrastructure metrics recommendation preference. Valid values: `Active`, `Inactive`.
        #[builder(into, default)]
        pub enhanced_infrastructure_metrics: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider of the external metrics recommendation preference. See External Metrics Preference below.
        #[builder(into, default)]
        pub external_metrics_preference: pulumi_wasm_rust::Output<
            Option<
                super::super::types::computeoptimizer::RecommendationPreferencesExternalMetricsPreference,
            >,
        >,
        /// The status of the inferred workload types recommendation preference. Valid values: `Active`, `Inactive`.
        #[builder(into, default)]
        pub inferred_workload_types: pulumi_wasm_rust::Output<Option<String>>,
        /// The preference to control the number of days the utilization metrics of the AWS resource are analyzed. Valid values: `DAYS_14`, `DAYS_32`, `DAYS_93`.
        #[builder(into, default)]
        pub look_back_period: pulumi_wasm_rust::Output<Option<String>>,
        /// The preference to control which resource type values are considered when generating rightsizing recommendations. See Preferred Resources below.
        #[builder(into, default)]
        pub preferred_resources: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::computeoptimizer::RecommendationPreferencesPreferredResource,
                >,
            >,
        >,
        /// The target resource type of the recommendation preferences. Valid values: `Ec2Instance`, `AutoScalingGroup`, `RdsDBInstance`.
        #[builder(into)]
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// The status of the savings estimation mode preference. Valid values: `AfterDiscounts`, `BeforeDiscounts`.
        #[builder(into, default)]
        pub savings_estimation_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The scope of the recommendation preferences. See Scope below.
        #[builder(into, default)]
        pub scope: pulumi_wasm_rust::Output<
            Option<super::super::types::computeoptimizer::RecommendationPreferencesScope>,
        >,
        /// The preference to control the resource’s CPU utilization threshold, CPU utilization headroom, and memory utilization headroom. See Utilization Preferences below.
        #[builder(into, default)]
        pub utilization_preferences: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::computeoptimizer::RecommendationPreferencesUtilizationPreference,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct RecommendationPreferencesResult {
        /// The status of the enhanced infrastructure metrics recommendation preference. Valid values: `Active`, `Inactive`.
        pub enhanced_infrastructure_metrics: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider of the external metrics recommendation preference. See External Metrics Preference below.
        pub external_metrics_preference: pulumi_wasm_rust::Output<
            Option<
                super::super::types::computeoptimizer::RecommendationPreferencesExternalMetricsPreference,
            >,
        >,
        /// The status of the inferred workload types recommendation preference. Valid values: `Active`, `Inactive`.
        pub inferred_workload_types: pulumi_wasm_rust::Output<Option<String>>,
        /// The preference to control the number of days the utilization metrics of the AWS resource are analyzed. Valid values: `DAYS_14`, `DAYS_32`, `DAYS_93`.
        pub look_back_period: pulumi_wasm_rust::Output<String>,
        /// The preference to control which resource type values are considered when generating rightsizing recommendations. See Preferred Resources below.
        pub preferred_resources: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::computeoptimizer::RecommendationPreferencesPreferredResource,
                >,
            >,
        >,
        /// The target resource type of the recommendation preferences. Valid values: `Ec2Instance`, `AutoScalingGroup`, `RdsDBInstance`.
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// The status of the savings estimation mode preference. Valid values: `AfterDiscounts`, `BeforeDiscounts`.
        pub savings_estimation_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The scope of the recommendation preferences. See Scope below.
        pub scope: pulumi_wasm_rust::Output<
            Option<super::super::types::computeoptimizer::RecommendationPreferencesScope>,
        >,
        /// The preference to control the resource’s CPU utilization threshold, CPU utilization headroom, and memory utilization headroom. See Utilization Preferences below.
        pub utilization_preferences: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::computeoptimizer::RecommendationPreferencesUtilizationPreference,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RecommendationPreferencesArgs,
    ) -> RecommendationPreferencesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enhanced_infrastructure_metrics_binding = args
            .enhanced_infrastructure_metrics
            .get_inner();
        let external_metrics_preference_binding = args
            .external_metrics_preference
            .get_inner();
        let inferred_workload_types_binding = args.inferred_workload_types.get_inner();
        let look_back_period_binding = args.look_back_period.get_inner();
        let preferred_resources_binding = args.preferred_resources.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
        let savings_estimation_mode_binding = args.savings_estimation_mode.get_inner();
        let scope_binding = args.scope.get_inner();
        let utilization_preferences_binding = args.utilization_preferences.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:computeoptimizer/recommendationPreferences:RecommendationPreferences"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enhancedInfrastructureMetrics".into(),
                    value: &enhanced_infrastructure_metrics_binding,
                },
                register_interface::ObjectField {
                    name: "externalMetricsPreference".into(),
                    value: &external_metrics_preference_binding,
                },
                register_interface::ObjectField {
                    name: "inferredWorkloadTypes".into(),
                    value: &inferred_workload_types_binding,
                },
                register_interface::ObjectField {
                    name: "lookBackPeriod".into(),
                    value: &look_back_period_binding,
                },
                register_interface::ObjectField {
                    name: "preferredResources".into(),
                    value: &preferred_resources_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
                register_interface::ObjectField {
                    name: "savingsEstimationMode".into(),
                    value: &savings_estimation_mode_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "utilizationPreferences".into(),
                    value: &utilization_preferences_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enhancedInfrastructureMetrics".into(),
                },
                register_interface::ResultField {
                    name: "externalMetricsPreference".into(),
                },
                register_interface::ResultField {
                    name: "inferredWorkloadTypes".into(),
                },
                register_interface::ResultField {
                    name: "lookBackPeriod".into(),
                },
                register_interface::ResultField {
                    name: "preferredResources".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "savingsEstimationMode".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "utilizationPreferences".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RecommendationPreferencesResult {
            enhanced_infrastructure_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enhancedInfrastructureMetrics").unwrap(),
            ),
            external_metrics_preference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalMetricsPreference").unwrap(),
            ),
            inferred_workload_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inferredWorkloadTypes").unwrap(),
            ),
            look_back_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lookBackPeriod").unwrap(),
            ),
            preferred_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredResources").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            savings_estimation_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("savingsEstimationMode").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            utilization_preferences: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("utilizationPreferences").unwrap(),
            ),
        }
    }
}