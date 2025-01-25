/// Provides a resource to manage EC2 Fleets.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod fleet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// Reserved.
        #[builder(into, default)]
        pub context: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether running instances should be terminated if the total target capacity of the EC2 Fleet is decreased below the current size of the EC2. Valid values: `no-termination`, `termination`. Defaults to `termination`. Supported only for fleets of type `maintain`.
        #[builder(into, default)]
        pub excess_capacity_termination_policy: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Information about the instances that were launched by the fleet. Available only when `type` is set to `instant`.
        #[builder(into, default)]
        pub fleet_instance_sets: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::FleetFleetInstanceSet>>,
        >,
        /// The state of the EC2 Fleet.
        #[builder(into, default)]
        pub fleet_state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The number of units fulfilled by this request compared to the set target capacity.
        #[builder(into, default)]
        pub fulfilled_capacity: pulumi_wasm_rust::InputOrOutput<Option<f64>>,
        /// The number of units fulfilled by this request compared to the set target On-Demand capacity.
        #[builder(into, default)]
        pub fulfilled_on_demand_capacity: pulumi_wasm_rust::InputOrOutput<Option<f64>>,
        /// Nested argument containing EC2 Launch Template configurations. Defined below.
        #[builder(into)]
        pub launch_template_configs: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::ec2::FleetLaunchTemplateConfig>,
        >,
        /// Nested argument containing On-Demand configurations. Defined below.
        #[builder(into, default)]
        pub on_demand_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::FleetOnDemandOptions>,
        >,
        /// Whether EC2 Fleet should replace unhealthy instances. Defaults to `false`. Supported only for fleets of type `maintain`.
        #[builder(into, default)]
        pub replace_unhealthy_instances: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Nested argument containing Spot configurations. Defined below.
        #[builder(into, default)]
        pub spot_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::FleetSpotOptions>,
        >,
        /// Map of Fleet tags. To tag instances at launch, specify the tags in the Launch Template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Nested argument containing target capacity configurations. Defined below.
        #[builder(into)]
        pub target_capacity_specification: pulumi_wasm_rust::InputOrOutput<
            super::super::types::ec2::FleetTargetCapacitySpecification,
        >,
        /// Whether to terminate instances for an EC2 Fleet if it is deleted successfully. Defaults to `false`.
        #[builder(into, default)]
        pub terminate_instances: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether running instances should be terminated when the EC2 Fleet expires. Defaults to `false`.
        #[builder(into, default)]
        pub terminate_instances_with_expiration: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The type of request. Indicates whether the EC2 Fleet only requests the target capacity, or also attempts to maintain it. Valid values: `maintain`, `request`, `instant`. Defaults to `maintain`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The start date and time of the request, in UTC format (for example, YYYY-MM-DDTHH:MM:SSZ). The default is to start fulfilling the request immediately.
        #[builder(into, default)]
        pub valid_from: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The end date and time of the request, in UTC format (for example, YYYY-MM-DDTHH:MM:SSZ). At this point, no new EC2 Fleet requests are placed or able to fulfill the request. If no value is specified, the request remains until you cancel it.
        #[builder(into, default)]
        pub valid_until: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// The ARN of the fleet
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Reserved.
        pub context: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether running instances should be terminated if the total target capacity of the EC2 Fleet is decreased below the current size of the EC2. Valid values: `no-termination`, `termination`. Defaults to `termination`. Supported only for fleets of type `maintain`.
        pub excess_capacity_termination_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Information about the instances that were launched by the fleet. Available only when `type` is set to `instant`.
        pub fleet_instance_sets: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::FleetFleetInstanceSet>,
        >,
        /// The state of the EC2 Fleet.
        pub fleet_state: pulumi_wasm_rust::Output<String>,
        /// The number of units fulfilled by this request compared to the set target capacity.
        pub fulfilled_capacity: pulumi_wasm_rust::Output<f64>,
        /// The number of units fulfilled by this request compared to the set target On-Demand capacity.
        pub fulfilled_on_demand_capacity: pulumi_wasm_rust::Output<f64>,
        /// Nested argument containing EC2 Launch Template configurations. Defined below.
        pub launch_template_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::FleetLaunchTemplateConfig>,
        >,
        /// Nested argument containing On-Demand configurations. Defined below.
        pub on_demand_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::FleetOnDemandOptions>,
        >,
        /// Whether EC2 Fleet should replace unhealthy instances. Defaults to `false`. Supported only for fleets of type `maintain`.
        pub replace_unhealthy_instances: pulumi_wasm_rust::Output<Option<bool>>,
        /// Nested argument containing Spot configurations. Defined below.
        pub spot_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::FleetSpotOptions>,
        >,
        /// Map of Fleet tags. To tag instances at launch, specify the tags in the Launch Template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Nested argument containing target capacity configurations. Defined below.
        pub target_capacity_specification: pulumi_wasm_rust::Output<
            super::super::types::ec2::FleetTargetCapacitySpecification,
        >,
        /// Whether to terminate instances for an EC2 Fleet if it is deleted successfully. Defaults to `false`.
        pub terminate_instances: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether running instances should be terminated when the EC2 Fleet expires. Defaults to `false`.
        pub terminate_instances_with_expiration: pulumi_wasm_rust::Output<Option<bool>>,
        /// The type of request. Indicates whether the EC2 Fleet only requests the target capacity, or also attempts to maintain it. Valid values: `maintain`, `request`, `instant`. Defaults to `maintain`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The start date and time of the request, in UTC format (for example, YYYY-MM-DDTHH:MM:SSZ). The default is to start fulfilling the request immediately.
        pub valid_from: pulumi_wasm_rust::Output<Option<String>>,
        /// The end date and time of the request, in UTC format (for example, YYYY-MM-DDTHH:MM:SSZ). At this point, no new EC2 Fleet requests are placed or able to fulfill the request. If no value is specified, the request remains until you cancel it.
        pub valid_until: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FleetArgs,
    ) -> FleetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let context_binding = args.context.get_output(context).get_inner();
        let excess_capacity_termination_policy_binding = args
            .excess_capacity_termination_policy
            .get_output(context)
            .get_inner();
        let fleet_instance_sets_binding = args
            .fleet_instance_sets
            .get_output(context)
            .get_inner();
        let fleet_state_binding = args.fleet_state.get_output(context).get_inner();
        let fulfilled_capacity_binding = args
            .fulfilled_capacity
            .get_output(context)
            .get_inner();
        let fulfilled_on_demand_capacity_binding = args
            .fulfilled_on_demand_capacity
            .get_output(context)
            .get_inner();
        let launch_template_configs_binding = args
            .launch_template_configs
            .get_output(context)
            .get_inner();
        let on_demand_options_binding = args
            .on_demand_options
            .get_output(context)
            .get_inner();
        let replace_unhealthy_instances_binding = args
            .replace_unhealthy_instances
            .get_output(context)
            .get_inner();
        let spot_options_binding = args.spot_options.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_capacity_specification_binding = args
            .target_capacity_specification
            .get_output(context)
            .get_inner();
        let terminate_instances_binding = args
            .terminate_instances
            .get_output(context)
            .get_inner();
        let terminate_instances_with_expiration_binding = args
            .terminate_instances_with_expiration
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let valid_from_binding = args.valid_from.get_output(context).get_inner();
        let valid_until_binding = args.valid_until.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/fleet:Fleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "context".into(),
                    value: &context_binding,
                },
                register_interface::ObjectField {
                    name: "excessCapacityTerminationPolicy".into(),
                    value: &excess_capacity_termination_policy_binding,
                },
                register_interface::ObjectField {
                    name: "fleetInstanceSets".into(),
                    value: &fleet_instance_sets_binding,
                },
                register_interface::ObjectField {
                    name: "fleetState".into(),
                    value: &fleet_state_binding,
                },
                register_interface::ObjectField {
                    name: "fulfilledCapacity".into(),
                    value: &fulfilled_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "fulfilledOnDemandCapacity".into(),
                    value: &fulfilled_on_demand_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "launchTemplateConfigs".into(),
                    value: &launch_template_configs_binding,
                },
                register_interface::ObjectField {
                    name: "onDemandOptions".into(),
                    value: &on_demand_options_binding,
                },
                register_interface::ObjectField {
                    name: "replaceUnhealthyInstances".into(),
                    value: &replace_unhealthy_instances_binding,
                },
                register_interface::ObjectField {
                    name: "spotOptions".into(),
                    value: &spot_options_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetCapacitySpecification".into(),
                    value: &target_capacity_specification_binding,
                },
                register_interface::ObjectField {
                    name: "terminateInstances".into(),
                    value: &terminate_instances_binding,
                },
                register_interface::ObjectField {
                    name: "terminateInstancesWithExpiration".into(),
                    value: &terminate_instances_with_expiration_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "validFrom".into(),
                    value: &valid_from_binding,
                },
                register_interface::ObjectField {
                    name: "validUntil".into(),
                    value: &valid_until_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "context".into(),
                },
                register_interface::ResultField {
                    name: "excessCapacityTerminationPolicy".into(),
                },
                register_interface::ResultField {
                    name: "fleetInstanceSets".into(),
                },
                register_interface::ResultField {
                    name: "fleetState".into(),
                },
                register_interface::ResultField {
                    name: "fulfilledCapacity".into(),
                },
                register_interface::ResultField {
                    name: "fulfilledOnDemandCapacity".into(),
                },
                register_interface::ResultField {
                    name: "launchTemplateConfigs".into(),
                },
                register_interface::ResultField {
                    name: "onDemandOptions".into(),
                },
                register_interface::ResultField {
                    name: "replaceUnhealthyInstances".into(),
                },
                register_interface::ResultField {
                    name: "spotOptions".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetCapacitySpecification".into(),
                },
                register_interface::ResultField {
                    name: "terminateInstances".into(),
                },
                register_interface::ResultField {
                    name: "terminateInstancesWithExpiration".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "validFrom".into(),
                },
                register_interface::ResultField {
                    name: "validUntil".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FleetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            context: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("context").unwrap(),
            ),
            excess_capacity_termination_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excessCapacityTerminationPolicy").unwrap(),
            ),
            fleet_instance_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleetInstanceSets").unwrap(),
            ),
            fleet_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleetState").unwrap(),
            ),
            fulfilled_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fulfilledCapacity").unwrap(),
            ),
            fulfilled_on_demand_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fulfilledOnDemandCapacity").unwrap(),
            ),
            launch_template_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchTemplateConfigs").unwrap(),
            ),
            on_demand_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onDemandOptions").unwrap(),
            ),
            replace_unhealthy_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replaceUnhealthyInstances").unwrap(),
            ),
            spot_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spotOptions").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_capacity_specification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetCapacitySpecification").unwrap(),
            ),
            terminate_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminateInstances").unwrap(),
            ),
            terminate_instances_with_expiration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminateInstancesWithExpiration").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            valid_from: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validFrom").unwrap(),
            ),
            valid_until: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validUntil").unwrap(),
            ),
        }
    }
}
