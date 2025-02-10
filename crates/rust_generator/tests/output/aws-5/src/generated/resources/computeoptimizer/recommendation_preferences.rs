/// Manages AWS Compute Optimizer recommendation preferences.
///
/// ## Example Usage
///
/// ### Lookback Period Preference
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod recommendation_preferences {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecommendationPreferencesArgs {
        /// The status of the enhanced infrastructure metrics recommendation preference. Valid values: `Active`, `Inactive`.
        #[builder(into, default)]
        pub enhanced_infrastructure_metrics: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The provider of the external metrics recommendation preference. See External Metrics Preference below.
        #[builder(into, default)]
        pub external_metrics_preference: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::computeoptimizer::RecommendationPreferencesExternalMetricsPreference,
            >,
        >,
        /// The status of the inferred workload types recommendation preference. Valid values: `Active`, `Inactive`.
        #[builder(into, default)]
        pub inferred_workload_types: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The preference to control the number of days the utilization metrics of the AWS resource are analyzed. Valid values: `DAYS_14`, `DAYS_32`, `DAYS_93`.
        #[builder(into, default)]
        pub look_back_period: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The preference to control which resource type values are considered when generating rightsizing recommendations. See Preferred Resources below.
        #[builder(into, default)]
        pub preferred_resources: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::computeoptimizer::RecommendationPreferencesPreferredResource,
                >,
            >,
        >,
        /// The target resource type of the recommendation preferences. Valid values: `Ec2Instance`, `AutoScalingGroup`, `RdsDBInstance`.
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The status of the savings estimation mode preference. Valid values: `AfterDiscounts`, `BeforeDiscounts`.
        #[builder(into, default)]
        pub savings_estimation_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scope of the recommendation preferences. See Scope below.
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::computeoptimizer::RecommendationPreferencesScope>,
        >,
        /// The preference to control the resource’s CPU utilization threshold, CPU utilization headroom, and memory utilization headroom. See Utilization Preferences below.
        #[builder(into, default)]
        pub utilization_preferences: pulumi_gestalt_rust::InputOrOutput<
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
        pub enhanced_infrastructure_metrics: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider of the external metrics recommendation preference. See External Metrics Preference below.
        pub external_metrics_preference: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::computeoptimizer::RecommendationPreferencesExternalMetricsPreference,
            >,
        >,
        /// The status of the inferred workload types recommendation preference. Valid values: `Active`, `Inactive`.
        pub inferred_workload_types: pulumi_gestalt_rust::Output<Option<String>>,
        /// The preference to control the number of days the utilization metrics of the AWS resource are analyzed. Valid values: `DAYS_14`, `DAYS_32`, `DAYS_93`.
        pub look_back_period: pulumi_gestalt_rust::Output<String>,
        /// The preference to control which resource type values are considered when generating rightsizing recommendations. See Preferred Resources below.
        pub preferred_resources: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::computeoptimizer::RecommendationPreferencesPreferredResource,
                >,
            >,
        >,
        /// The target resource type of the recommendation preferences. Valid values: `Ec2Instance`, `AutoScalingGroup`, `RdsDBInstance`.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// The status of the savings estimation mode preference. Valid values: `AfterDiscounts`, `BeforeDiscounts`.
        pub savings_estimation_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The scope of the recommendation preferences. See Scope below.
        pub scope: pulumi_gestalt_rust::Output<
            Option<super::super::types::computeoptimizer::RecommendationPreferencesScope>,
        >,
        /// The preference to control the resource’s CPU utilization threshold, CPU utilization headroom, and memory utilization headroom. See Utilization Preferences below.
        pub utilization_preferences: pulumi_gestalt_rust::Output<
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecommendationPreferencesArgs,
    ) -> RecommendationPreferencesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enhanced_infrastructure_metrics_binding = args
            .enhanced_infrastructure_metrics
            .get_output(context);
        let external_metrics_preference_binding = args
            .external_metrics_preference
            .get_output(context);
        let inferred_workload_types_binding = args
            .inferred_workload_types
            .get_output(context);
        let look_back_period_binding = args.look_back_period.get_output(context);
        let preferred_resources_binding = args.preferred_resources.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let savings_estimation_mode_binding = args
            .savings_estimation_mode
            .get_output(context);
        let scope_binding = args.scope.get_output(context);
        let utilization_preferences_binding = args
            .utilization_preferences
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:computeoptimizer/recommendationPreferences:RecommendationPreferences"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enhancedInfrastructureMetrics".into(),
                    value: enhanced_infrastructure_metrics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "externalMetricsPreference".into(),
                    value: external_metrics_preference_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inferredWorkloadTypes".into(),
                    value: inferred_workload_types_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lookBackPeriod".into(),
                    value: look_back_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredResources".into(),
                    value: preferred_resources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: resource_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "savingsEstimationMode".into(),
                    value: savings_estimation_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "utilizationPreferences".into(),
                    value: utilization_preferences_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RecommendationPreferencesResult {
            enhanced_infrastructure_metrics: o
                .get_field("enhancedInfrastructureMetrics"),
            external_metrics_preference: o.get_field("externalMetricsPreference"),
            inferred_workload_types: o.get_field("inferredWorkloadTypes"),
            look_back_period: o.get_field("lookBackPeriod"),
            preferred_resources: o.get_field("preferredResources"),
            resource_type: o.get_field("resourceType"),
            savings_estimation_mode: o.get_field("savingsEstimationMode"),
            scope: o.get_field("scope"),
            utilization_preferences: o.get_field("utilizationPreferences"),
        }
    }
}
