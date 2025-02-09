/// Provides a resource to manage EC2 Fleets.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = fleet::create(
///         "example",
///         FleetArgs::builder()
///             .launch_template_configs(
///                 vec![
///                     FleetLaunchTemplateConfig::builder()
///                     .launchTemplateSpecification(FleetLaunchTemplateConfigLaunchTemplateSpecification::builder()
///                     .launchTemplateId("${exampleAwsLaunchTemplate.id}")
///                     .version("${exampleAwsLaunchTemplate.latestVersion}").build_struct())
///                     .build_struct(),
///                 ],
///             )
///             .target_capacity_specification(
///                 FleetTargetCapacitySpecification::builder()
///                     .defaultTargetCapacityType("spot")
///                     .totalTargetCapacity(5)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_fleet` using the Fleet identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/fleet:Fleet example fleet-b9b55d27-c5fc-41ac-a6f3-48fcc91f080c
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fleet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// Reserved.
        #[builder(into, default)]
        pub context: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether running instances should be terminated if the total target capacity of the EC2 Fleet is decreased below the current size of the EC2. Valid values: `no-termination`, `termination`. Defaults to `termination`. Supported only for fleets of type `maintain`.
        #[builder(into, default)]
        pub excess_capacity_termination_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Information about the instances that were launched by the fleet. Available only when `type` is set to `instant`.
        #[builder(into, default)]
        pub fleet_instance_sets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::FleetFleetInstanceSet>>,
        >,
        /// The state of the EC2 Fleet.
        #[builder(into, default)]
        pub fleet_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of units fulfilled by this request compared to the set target capacity.
        #[builder(into, default)]
        pub fulfilled_capacity: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The number of units fulfilled by this request compared to the set target On-Demand capacity.
        #[builder(into, default)]
        pub fulfilled_on_demand_capacity: pulumi_gestalt_rust::InputOrOutput<
            Option<f64>,
        >,
        /// Nested argument containing EC2 Launch Template configurations. Defined below.
        #[builder(into)]
        pub launch_template_configs: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::ec2::FleetLaunchTemplateConfig>,
        >,
        /// Nested argument containing On-Demand configurations. Defined below.
        #[builder(into, default)]
        pub on_demand_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::FleetOnDemandOptions>,
        >,
        /// Whether EC2 Fleet should replace unhealthy instances. Defaults to `false`. Supported only for fleets of type `maintain`.
        #[builder(into, default)]
        pub replace_unhealthy_instances: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Nested argument containing Spot configurations. Defined below.
        #[builder(into, default)]
        pub spot_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::FleetSpotOptions>,
        >,
        /// Map of Fleet tags. To tag instances at launch, specify the tags in the Launch Template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Nested argument containing target capacity configurations. Defined below.
        #[builder(into)]
        pub target_capacity_specification: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::ec2::FleetTargetCapacitySpecification,
        >,
        /// Whether to terminate instances for an EC2 Fleet if it is deleted successfully. Defaults to `false`.
        #[builder(into, default)]
        pub terminate_instances: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether running instances should be terminated when the EC2 Fleet expires. Defaults to `false`.
        #[builder(into, default)]
        pub terminate_instances_with_expiration: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The type of request. Indicates whether the EC2 Fleet only requests the target capacity, or also attempts to maintain it. Valid values: `maintain`, `request`, `instant`. Defaults to `maintain`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The start date and time of the request, in UTC format (for example, YYYY-MM-DDTHH:MM:SSZ). The default is to start fulfilling the request immediately.
        #[builder(into, default)]
        pub valid_from: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The end date and time of the request, in UTC format (for example, YYYY-MM-DDTHH:MM:SSZ). At this point, no new EC2 Fleet requests are placed or able to fulfill the request. If no value is specified, the request remains until you cancel it.
        #[builder(into, default)]
        pub valid_until: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// The ARN of the fleet
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Reserved.
        pub context: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether running instances should be terminated if the total target capacity of the EC2 Fleet is decreased below the current size of the EC2. Valid values: `no-termination`, `termination`. Defaults to `termination`. Supported only for fleets of type `maintain`.
        pub excess_capacity_termination_policy: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Information about the instances that were launched by the fleet. Available only when `type` is set to `instant`.
        pub fleet_instance_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::FleetFleetInstanceSet>,
        >,
        /// The state of the EC2 Fleet.
        pub fleet_state: pulumi_gestalt_rust::Output<String>,
        /// The number of units fulfilled by this request compared to the set target capacity.
        pub fulfilled_capacity: pulumi_gestalt_rust::Output<f64>,
        /// The number of units fulfilled by this request compared to the set target On-Demand capacity.
        pub fulfilled_on_demand_capacity: pulumi_gestalt_rust::Output<f64>,
        /// Nested argument containing EC2 Launch Template configurations. Defined below.
        pub launch_template_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::FleetLaunchTemplateConfig>,
        >,
        /// Nested argument containing On-Demand configurations. Defined below.
        pub on_demand_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::FleetOnDemandOptions>,
        >,
        /// Whether EC2 Fleet should replace unhealthy instances. Defaults to `false`. Supported only for fleets of type `maintain`.
        pub replace_unhealthy_instances: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Nested argument containing Spot configurations. Defined below.
        pub spot_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::FleetSpotOptions>,
        >,
        /// Map of Fleet tags. To tag instances at launch, specify the tags in the Launch Template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Nested argument containing target capacity configurations. Defined below.
        pub target_capacity_specification: pulumi_gestalt_rust::Output<
            super::super::types::ec2::FleetTargetCapacitySpecification,
        >,
        /// Whether to terminate instances for an EC2 Fleet if it is deleted successfully. Defaults to `false`.
        pub terminate_instances: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether running instances should be terminated when the EC2 Fleet expires. Defaults to `false`.
        pub terminate_instances_with_expiration: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The type of request. Indicates whether the EC2 Fleet only requests the target capacity, or also attempts to maintain it. Valid values: `maintain`, `request`, `instant`. Defaults to `maintain`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// The start date and time of the request, in UTC format (for example, YYYY-MM-DDTHH:MM:SSZ). The default is to start fulfilling the request immediately.
        pub valid_from: pulumi_gestalt_rust::Output<Option<String>>,
        /// The end date and time of the request, in UTC format (for example, YYYY-MM-DDTHH:MM:SSZ). At this point, no new EC2 Fleet requests are placed or able to fulfill the request. If no value is specified, the request remains until you cancel it.
        pub valid_until: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FleetArgs,
    ) -> FleetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let context_binding = args.context.get_output(context);
        let excess_capacity_termination_policy_binding = args
            .excess_capacity_termination_policy
            .get_output(context);
        let fleet_instance_sets_binding = args.fleet_instance_sets.get_output(context);
        let fleet_state_binding = args.fleet_state.get_output(context);
        let fulfilled_capacity_binding = args.fulfilled_capacity.get_output(context);
        let fulfilled_on_demand_capacity_binding = args
            .fulfilled_on_demand_capacity
            .get_output(context);
        let launch_template_configs_binding = args
            .launch_template_configs
            .get_output(context);
        let on_demand_options_binding = args.on_demand_options.get_output(context);
        let replace_unhealthy_instances_binding = args
            .replace_unhealthy_instances
            .get_output(context);
        let spot_options_binding = args.spot_options.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_capacity_specification_binding = args
            .target_capacity_specification
            .get_output(context);
        let terminate_instances_binding = args.terminate_instances.get_output(context);
        let terminate_instances_with_expiration_binding = args
            .terminate_instances_with_expiration
            .get_output(context);
        let type__binding = args.type_.get_output(context);
        let valid_from_binding = args.valid_from.get_output(context);
        let valid_until_binding = args.valid_until.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/fleet:Fleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "context".into(),
                    value: context_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excessCapacityTerminationPolicy".into(),
                    value: excess_capacity_termination_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fleetInstanceSets".into(),
                    value: fleet_instance_sets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fleetState".into(),
                    value: fleet_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fulfilledCapacity".into(),
                    value: fulfilled_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fulfilledOnDemandCapacity".into(),
                    value: fulfilled_on_demand_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "launchTemplateConfigs".into(),
                    value: launch_template_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onDemandOptions".into(),
                    value: on_demand_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replaceUnhealthyInstances".into(),
                    value: replace_unhealthy_instances_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spotOptions".into(),
                    value: spot_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetCapacitySpecification".into(),
                    value: target_capacity_specification_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "terminateInstances".into(),
                    value: terminate_instances_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "terminateInstancesWithExpiration".into(),
                    value: terminate_instances_with_expiration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validFrom".into(),
                    value: valid_from_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validUntil".into(),
                    value: valid_until_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FleetResult {
            arn: o.get_field("arn"),
            context: o.get_field("context"),
            excess_capacity_termination_policy: o
                .get_field("excessCapacityTerminationPolicy"),
            fleet_instance_sets: o.get_field("fleetInstanceSets"),
            fleet_state: o.get_field("fleetState"),
            fulfilled_capacity: o.get_field("fulfilledCapacity"),
            fulfilled_on_demand_capacity: o.get_field("fulfilledOnDemandCapacity"),
            launch_template_configs: o.get_field("launchTemplateConfigs"),
            on_demand_options: o.get_field("onDemandOptions"),
            replace_unhealthy_instances: o.get_field("replaceUnhealthyInstances"),
            spot_options: o.get_field("spotOptions"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_capacity_specification: o.get_field("targetCapacitySpecification"),
            terminate_instances: o.get_field("terminateInstances"),
            terminate_instances_with_expiration: o
                .get_field("terminateInstancesWithExpiration"),
            type_: o.get_field("type"),
            valid_from: o.get_field("validFrom"),
            valid_until: o.get_field("validUntil"),
        }
    }
}
