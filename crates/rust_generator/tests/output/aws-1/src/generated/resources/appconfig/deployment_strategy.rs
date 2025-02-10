/// Provides an AppConfig Deployment Strategy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:DeploymentStrategy
///     properties:
///       name: example-deployment-strategy-tf
///       description: Example Deployment Strategy
///       deploymentDurationInMinutes: 3
///       finalBakeTimeInMinutes: 4
///       growthFactor: 10
///       growthType: LINEAR
///       replicateTo: NONE
///       tags:
///         Type: AppConfig Deployment Strategy
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppConfig Deployment Strategies using their deployment strategy ID. For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/deploymentStrategy:DeploymentStrategy example 11xxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deployment_strategy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentStrategyArgs {
        /// Total amount of time for a deployment to last. Minimum value of 0, maximum value of 1440.
        #[builder(into)]
        pub deployment_duration_in_minutes: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Description of the deployment strategy. Can be at most 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amount of time AWS AppConfig monitors for alarms before considering the deployment to be complete and no longer eligible for automatic roll back. Minimum value of 0, maximum value of 1440.
        #[builder(into, default)]
        pub final_bake_time_in_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Percentage of targets to receive a deployed configuration during each interval. Minimum value of 1.0, maximum value of 100.0.
        #[builder(into)]
        pub growth_factor: pulumi_gestalt_rust::InputOrOutput<f64>,
        /// Algorithm used to define how percentage grows over time. Valid value: `LINEAR` and `EXPONENTIAL`. Defaults to `LINEAR`.
        #[builder(into, default)]
        pub growth_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name for the deployment strategy. Must be between 1 and 64 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Where to save the deployment strategy. Valid values: `NONE` and `SSM_DOCUMENT`.
        #[builder(into)]
        pub replicate_to: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentStrategyResult {
        /// ARN of the AppConfig Deployment Strategy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Total amount of time for a deployment to last. Minimum value of 0, maximum value of 1440.
        pub deployment_duration_in_minutes: pulumi_gestalt_rust::Output<i32>,
        /// Description of the deployment strategy. Can be at most 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amount of time AWS AppConfig monitors for alarms before considering the deployment to be complete and no longer eligible for automatic roll back. Minimum value of 0, maximum value of 1440.
        pub final_bake_time_in_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Percentage of targets to receive a deployed configuration during each interval. Minimum value of 1.0, maximum value of 100.0.
        pub growth_factor: pulumi_gestalt_rust::Output<f64>,
        /// Algorithm used to define how percentage grows over time. Valid value: `LINEAR` and `EXPONENTIAL`. Defaults to `LINEAR`.
        pub growth_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name for the deployment strategy. Must be between 1 and 64 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Where to save the deployment strategy. Valid values: `NONE` and `SSM_DOCUMENT`.
        pub replicate_to: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: DeploymentStrategyArgs,
    ) -> DeploymentStrategyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deployment_duration_in_minutes_binding = args
            .deployment_duration_in_minutes
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let final_bake_time_in_minutes_binding = args
            .final_bake_time_in_minutes
            .get_output(context);
        let growth_factor_binding = args.growth_factor.get_output(context);
        let growth_type_binding = args.growth_type.get_output(context);
        let name_binding = args.name.get_output(context);
        let replicate_to_binding = args.replicate_to.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appconfig/deploymentStrategy:DeploymentStrategy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentDurationInMinutes".into(),
                    value: deployment_duration_in_minutes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "finalBakeTimeInMinutes".into(),
                    value: final_bake_time_in_minutes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "growthFactor".into(),
                    value: growth_factor_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "growthType".into(),
                    value: growth_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicateTo".into(),
                    value: replicate_to_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeploymentStrategyResult {
            arn: o.get_field("arn"),
            deployment_duration_in_minutes: o.get_field("deploymentDurationInMinutes"),
            description: o.get_field("description"),
            final_bake_time_in_minutes: o.get_field("finalBakeTimeInMinutes"),
            growth_factor: o.get_field("growthFactor"),
            growth_type: o.get_field("growthType"),
            name: o.get_field("name"),
            replicate_to: o.get_field("replicateTo"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
