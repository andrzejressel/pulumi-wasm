/// Provides an EC2 Spot Fleet Request resource. This allows a fleet of Spot
/// instances to be requested on the Spot market.
///
/// > **NOTE [AWS strongly discourages](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-best-practices.html#which-spot-request-method-to-use) the use of the legacy APIs called by this resource.
/// We recommend using the EC2 Fleet or Auto Scaling Group resources instead.
///
/// ## Example Usage
///
/// ### Using launch specifications
///
/// ```yaml
/// resources:
///   # Request a Spot fleet
///   cheapCompute:
///     type: aws:ec2:SpotFleetRequest
///     name: cheap_compute
///     properties:
///       iamFleetRole: arn:aws:iam::12345678:role/spot-fleet
///       spotPrice: '0.03'
///       allocationStrategy: diversified
///       targetCapacity: 6
///       validUntil: 2019-11-04T20:44:20Z
///       launchSpecifications:
///         - instanceType: m4.10xlarge
///           ami: ami-1234
///           spotPrice: '2.793'
///           placementTenancy: dedicated
///           iamInstanceProfileArn: ${example.arn}
///         - instanceType: m4.4xlarge
///           ami: ami-5678
///           keyName: my-key
///           spotPrice: '1.117'
///           iamInstanceProfileArn: ${example.arn}
///           availabilityZone: us-west-1a
///           subnetId: subnet-1234
///           weightedCapacity: 35
///           rootBlockDevices:
///             - volumeSize: '300'
///               volumeType: gp2
///           tags:
///             Name: spot-fleet-example
/// ```
///
/// ### Using launch templates
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = launch_template::create(
///         "foo",
///         LaunchTemplateArgs::builder()
///             .image_id("ami-516b9131")
///             .instance_type("m1.small")
///             .key_name("some-key")
///             .name("launch-template")
///             .build_struct(),
///     );
///     let fooSpotFleetRequest = spot_fleet_request::create(
///         "fooSpotFleetRequest",
///         SpotFleetRequestArgs::builder()
///             .iam_fleet_role("arn:aws:iam::12345678:role/spot-fleet")
///             .launch_template_configs(
///                 vec![
///                     SpotFleetRequestLaunchTemplateConfig::builder()
///                     .launchTemplateSpecification(SpotFleetRequestLaunchTemplateConfigLaunchTemplateSpecification::builder()
///                     .id("${foo.id}").version("${foo.latestVersion}").build_struct())
///                     .build_struct(),
///                 ],
///             )
///             .spot_price("0.005")
///             .target_capacity(2)
///             .valid_until("2019-11-04T20:44:20Z")
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **NOTE:** This provider does not support the functionality where multiple `subnet_id` or `availability_zone` parameters can be specified in the same
/// launch configuration block. If you want to specify multiple values, then separate launch configuration blocks should be used or launch template overrides should be configured, one per subnet:
///
/// ### Using multiple launch specifications
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = spot_fleet_request::create(
///         "foo",
///         SpotFleetRequestArgs::builder()
///             .iam_fleet_role("arn:aws:iam::12345678:role/spot-fleet")
///             .launch_specifications(
///                 vec![
///                     SpotFleetRequestLaunchSpecification::builder().ami("ami-d06a90b0")
///                     .availabilityZone("us-west-2a").instanceType("m1.small")
///                     .keyName("my-key").build_struct(),
///                     SpotFleetRequestLaunchSpecification::builder().ami("ami-d06a90b0")
///                     .availabilityZone("us-west-2a").instanceType("m5.large")
///                     .keyName("my-key").build_struct(),
///                 ],
///             )
///             .spot_price("0.005")
///             .target_capacity(2)
///             .valid_until("2019-11-04T20:44:20Z")
///             .build_struct(),
///     );
/// }
/// ```
///
/// > In this example, we use a `dynamic` block to define zero or more `launch_specification` blocks, producing one for each element in the list of subnet ids.
///
///
/// ### Using multiple launch configurations
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:ec2:LaunchTemplate
///     properties:
///       name: launch-template
///       imageId: ami-516b9131
///       instanceType: m1.small
///       keyName: some-key
///   fooSpotFleetRequest:
///     type: aws:ec2:SpotFleetRequest
///     name: foo
///     properties:
///       iamFleetRole: arn:aws:iam::12345678:role/spot-fleet
///       spotPrice: '0.005'
///       targetCapacity: 2
///       validUntil: 2019-11-04T20:44:20Z
///       launchTemplateConfigs:
///         - launchTemplateSpecification:
///             id: ${foo.id}
///             version: ${foo.latestVersion}
///           overrides:
///             - subnetId: ${example.ids[0]}
///             - subnetId: ${example.ids[1]}
///             - subnetId: ${example.ids[2]}
///     options:
///       dependsOn:
///         - ${["test-attach"]}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ec2:getSubnets
///       arguments:
///         filters:
///           - name: vpc-id
///             values:
///               - ${vpcId}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Spot Fleet Requests using `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/spotFleetRequest:SpotFleetRequest fleet sfr-005e9ec8-5546-4c31-b317-31a62325411e
/// ```
pub mod spot_fleet_request {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpotFleetRequestArgs {
        /// Indicates how to allocate the target capacity across
        /// the Spot pools specified by the Spot fleet request. Valid values: `lowestPrice`, `diversified`, `capacityOptimized`, `capacityOptimizedPrioritized`, and `priceCapacityOptimized`. The default is
        /// `lowestPrice`.
        #[builder(into, default)]
        pub allocation_strategy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Reserved.
        #[builder(into, default)]
        pub context: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether running Spot
        /// instances should be terminated if the target capacity of the Spot fleet
        /// request is decreased below the current size of the Spot fleet.
        #[builder(into, default)]
        pub excess_capacity_termination_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The type of fleet request. Indicates whether the Spot Fleet only requests the target
        /// capacity or also attempts to maintain it. Default is `maintain`.
        #[builder(into, default)]
        pub fleet_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Grants the Spot fleet permission to terminate
        /// Spot instances on your behalf when you cancel its Spot fleet request using
        /// CancelSpotFleetRequests or when the Spot fleet request expires, if you set
        /// terminateInstancesWithExpiration.
        #[builder(into)]
        pub iam_fleet_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether a Spot
        /// instance stops or terminates when it is interrupted. Default is
        /// `terminate`.
        #[builder(into, default)]
        pub instance_interruption_behaviour: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The number of Spot pools across which to allocate your target Spot capacity.
        /// Valid only when `allocation_strategy` is set to `lowestPrice`. Spot Fleet selects
        /// the cheapest Spot pools and evenly allocates your target Spot capacity across
        /// the number of Spot pools that you specify.
        #[builder(into, default)]
        pub instance_pools_to_use_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Used to define the launch configuration of the
        /// spot-fleet request. Can be specified multiple times to define different bids
        /// across different markets and instance types. Conflicts with `launch_template_config`. At least one of `launch_specification` or `launch_template_config` is required.
        ///
        /// **Note**: This takes in similar but not
        /// identical inputs as `aws.ec2.Instance`.  There are limitations on
        /// what you can specify. See the list of officially supported inputs in the
        /// [reference documentation](http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_SpotFleetLaunchSpecification.html). Any normal `aws.ec2.Instance` parameter that corresponds to those inputs may be used and it have
        /// a additional parameter `iam_instance_profile_arn` takes `aws.iam.InstanceProfile` attribute `arn` as input.
        #[builder(into, default)]
        pub launch_specifications: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecification>>,
        >,
        /// Launch template configuration block. See Launch Template Configs below for more details. Conflicts with `launch_specification`. At least one of `launch_specification` or `launch_template_config` is required.
        #[builder(into, default)]
        pub launch_template_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchTemplateConfig>>,
        >,
        /// A list of elastic load balancer names to add to the Spot fleet.
        #[builder(into, default)]
        pub load_balancers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The order of the launch template overrides to use in fulfilling On-Demand capacity. the possible values are: `lowestPrice` and `prioritized`. the default is `lowestPrice`.
        #[builder(into, default)]
        pub on_demand_allocation_strategy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The maximum amount per hour for On-Demand Instances that you're willing to pay. When the maximum amount you're willing to pay is reached, the fleet stops launching instances even if it hasn’t met the target capacity.
        #[builder(into, default)]
        pub on_demand_max_total_price: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The number of On-Demand units to request. If the request type is `maintain`, you can specify a target capacity of 0 and add capacity later.
        #[builder(into, default)]
        pub on_demand_target_capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Indicates whether Spot fleet should replace unhealthy instances. Default `false`.
        #[builder(into, default)]
        pub replace_unhealthy_instances: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Nested argument containing maintenance strategies for managing your Spot Instances that are at an elevated risk of being interrupted. Defined below.
        #[builder(into, default)]
        pub spot_maintenance_strategies: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::SpotFleetRequestSpotMaintenanceStrategies>,
        >,
        /// The maximum bid price per unit hour.
        #[builder(into, default)]
        pub spot_price: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The number of units to request. You can choose to set the
        /// target capacity in terms of instances or a performance characteristic that is
        /// important to your application workload, such as vCPUs, memory, or I/O.
        #[builder(into)]
        pub target_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The unit for the target capacity. This can only be done with `instance_requirements` defined
        #[builder(into, default)]
        pub target_capacity_unit_type: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of `aws.alb.TargetGroup` ARNs, for use with Application Load Balancing.
        #[builder(into, default)]
        pub target_group_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Indicates whether running Spot
        /// instances should be terminated when the resource is deleted (and the Spot fleet request cancelled).
        /// If no value is specified, the value of the `terminate_instances_with_expiration` argument is used.
        #[builder(into, default)]
        pub terminate_instances_on_delete: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Indicates whether running Spot
        /// instances should be terminated when the Spot fleet request expires.
        #[builder(into, default)]
        pub terminate_instances_with_expiration: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The start date and time of the request, in UTC [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.8) format(for example, YYYY-MM-DDTHH:MM:SSZ). The default is to start fulfilling the request immediately.
        #[builder(into, default)]
        pub valid_from: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The end date and time of the request, in UTC [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.8) format(for example, YYYY-MM-DDTHH:MM:SSZ). At this point, no new Spot instance requests are placed or enabled to fulfill the request.
        #[builder(into, default)]
        pub valid_until: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set, this provider will
        /// wait for the Spot Request to be fulfilled, and will throw an error if the
        /// timeout of 10m is reached.
        #[builder(into, default)]
        pub wait_for_fulfillment: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpotFleetRequestResult {
        /// Indicates how to allocate the target capacity across
        /// the Spot pools specified by the Spot fleet request. Valid values: `lowestPrice`, `diversified`, `capacityOptimized`, `capacityOptimizedPrioritized`, and `priceCapacityOptimized`. The default is
        /// `lowestPrice`.
        pub allocation_strategy: pulumi_gestalt_rust::Output<Option<String>>,
        pub client_token: pulumi_gestalt_rust::Output<String>,
        /// Reserved.
        pub context: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether running Spot
        /// instances should be terminated if the target capacity of the Spot fleet
        /// request is decreased below the current size of the Spot fleet.
        pub excess_capacity_termination_policy: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The type of fleet request. Indicates whether the Spot Fleet only requests the target
        /// capacity or also attempts to maintain it. Default is `maintain`.
        pub fleet_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Grants the Spot fleet permission to terminate
        /// Spot instances on your behalf when you cancel its Spot fleet request using
        /// CancelSpotFleetRequests or when the Spot fleet request expires, if you set
        /// terminateInstancesWithExpiration.
        pub iam_fleet_role: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether a Spot
        /// instance stops or terminates when it is interrupted. Default is
        /// `terminate`.
        pub instance_interruption_behaviour: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of Spot pools across which to allocate your target Spot capacity.
        /// Valid only when `allocation_strategy` is set to `lowestPrice`. Spot Fleet selects
        /// the cheapest Spot pools and evenly allocates your target Spot capacity across
        /// the number of Spot pools that you specify.
        pub instance_pools_to_use_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Used to define the launch configuration of the
        /// spot-fleet request. Can be specified multiple times to define different bids
        /// across different markets and instance types. Conflicts with `launch_template_config`. At least one of `launch_specification` or `launch_template_config` is required.
        ///
        /// **Note**: This takes in similar but not
        /// identical inputs as `aws.ec2.Instance`.  There are limitations on
        /// what you can specify. See the list of officially supported inputs in the
        /// [reference documentation](http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_SpotFleetLaunchSpecification.html). Any normal `aws.ec2.Instance` parameter that corresponds to those inputs may be used and it have
        /// a additional parameter `iam_instance_profile_arn` takes `aws.iam.InstanceProfile` attribute `arn` as input.
        pub launch_specifications: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecification>>,
        >,
        /// Launch template configuration block. See Launch Template Configs below for more details. Conflicts with `launch_specification`. At least one of `launch_specification` or `launch_template_config` is required.
        pub launch_template_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchTemplateConfig>>,
        >,
        /// A list of elastic load balancer names to add to the Spot fleet.
        pub load_balancers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The order of the launch template overrides to use in fulfilling On-Demand capacity. the possible values are: `lowestPrice` and `prioritized`. the default is `lowestPrice`.
        pub on_demand_allocation_strategy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The maximum amount per hour for On-Demand Instances that you're willing to pay. When the maximum amount you're willing to pay is reached, the fleet stops launching instances even if it hasn’t met the target capacity.
        pub on_demand_max_total_price: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of On-Demand units to request. If the request type is `maintain`, you can specify a target capacity of 0 and add capacity later.
        pub on_demand_target_capacity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Indicates whether Spot fleet should replace unhealthy instances. Default `false`.
        pub replace_unhealthy_instances: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Nested argument containing maintenance strategies for managing your Spot Instances that are at an elevated risk of being interrupted. Defined below.
        pub spot_maintenance_strategies: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::SpotFleetRequestSpotMaintenanceStrategies>,
        >,
        /// The maximum bid price per unit hour.
        pub spot_price: pulumi_gestalt_rust::Output<Option<String>>,
        /// The state of the Spot fleet request.
        pub spot_request_state: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The number of units to request. You can choose to set the
        /// target capacity in terms of instances or a performance characteristic that is
        /// important to your application workload, such as vCPUs, memory, or I/O.
        pub target_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The unit for the target capacity. This can only be done with `instance_requirements` defined
        pub target_capacity_unit_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of `aws.alb.TargetGroup` ARNs, for use with Application Load Balancing.
        pub target_group_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Indicates whether running Spot
        /// instances should be terminated when the resource is deleted (and the Spot fleet request cancelled).
        /// If no value is specified, the value of the `terminate_instances_with_expiration` argument is used.
        pub terminate_instances_on_delete: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether running Spot
        /// instances should be terminated when the Spot fleet request expires.
        pub terminate_instances_with_expiration: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The start date and time of the request, in UTC [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.8) format(for example, YYYY-MM-DDTHH:MM:SSZ). The default is to start fulfilling the request immediately.
        pub valid_from: pulumi_gestalt_rust::Output<Option<String>>,
        /// The end date and time of the request, in UTC [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.8) format(for example, YYYY-MM-DDTHH:MM:SSZ). At this point, no new Spot instance requests are placed or enabled to fulfill the request.
        pub valid_until: pulumi_gestalt_rust::Output<Option<String>>,
        /// If set, this provider will
        /// wait for the Spot Request to be fulfilled, and will throw an error if the
        /// timeout of 10m is reached.
        pub wait_for_fulfillment: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SpotFleetRequestArgs,
    ) -> SpotFleetRequestResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let allocation_strategy_binding = args
            .allocation_strategy
            .get_output(context)
            .get_inner();
        let context_binding = args.context.get_output(context).get_inner();
        let excess_capacity_termination_policy_binding = args
            .excess_capacity_termination_policy
            .get_output(context)
            .get_inner();
        let fleet_type_binding = args.fleet_type.get_output(context).get_inner();
        let iam_fleet_role_binding = args.iam_fleet_role.get_output(context).get_inner();
        let instance_interruption_behaviour_binding = args
            .instance_interruption_behaviour
            .get_output(context)
            .get_inner();
        let instance_pools_to_use_count_binding = args
            .instance_pools_to_use_count
            .get_output(context)
            .get_inner();
        let launch_specifications_binding = args
            .launch_specifications
            .get_output(context)
            .get_inner();
        let launch_template_configs_binding = args
            .launch_template_configs
            .get_output(context)
            .get_inner();
        let load_balancers_binding = args.load_balancers.get_output(context).get_inner();
        let on_demand_allocation_strategy_binding = args
            .on_demand_allocation_strategy
            .get_output(context)
            .get_inner();
        let on_demand_max_total_price_binding = args
            .on_demand_max_total_price
            .get_output(context)
            .get_inner();
        let on_demand_target_capacity_binding = args
            .on_demand_target_capacity
            .get_output(context)
            .get_inner();
        let replace_unhealthy_instances_binding = args
            .replace_unhealthy_instances
            .get_output(context)
            .get_inner();
        let spot_maintenance_strategies_binding = args
            .spot_maintenance_strategies
            .get_output(context)
            .get_inner();
        let spot_price_binding = args.spot_price.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_capacity_binding = args
            .target_capacity
            .get_output(context)
            .get_inner();
        let target_capacity_unit_type_binding = args
            .target_capacity_unit_type
            .get_output(context)
            .get_inner();
        let target_group_arns_binding = args
            .target_group_arns
            .get_output(context)
            .get_inner();
        let terminate_instances_on_delete_binding = args
            .terminate_instances_on_delete
            .get_output(context)
            .get_inner();
        let terminate_instances_with_expiration_binding = args
            .terminate_instances_with_expiration
            .get_output(context)
            .get_inner();
        let valid_from_binding = args.valid_from.get_output(context).get_inner();
        let valid_until_binding = args.valid_until.get_output(context).get_inner();
        let wait_for_fulfillment_binding = args
            .wait_for_fulfillment
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/spotFleetRequest:SpotFleetRequest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocationStrategy".into(),
                    value: &allocation_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "context".into(),
                    value: &context_binding,
                },
                register_interface::ObjectField {
                    name: "excessCapacityTerminationPolicy".into(),
                    value: &excess_capacity_termination_policy_binding,
                },
                register_interface::ObjectField {
                    name: "fleetType".into(),
                    value: &fleet_type_binding,
                },
                register_interface::ObjectField {
                    name: "iamFleetRole".into(),
                    value: &iam_fleet_role_binding,
                },
                register_interface::ObjectField {
                    name: "instanceInterruptionBehaviour".into(),
                    value: &instance_interruption_behaviour_binding,
                },
                register_interface::ObjectField {
                    name: "instancePoolsToUseCount".into(),
                    value: &instance_pools_to_use_count_binding,
                },
                register_interface::ObjectField {
                    name: "launchSpecifications".into(),
                    value: &launch_specifications_binding,
                },
                register_interface::ObjectField {
                    name: "launchTemplateConfigs".into(),
                    value: &launch_template_configs_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancers".into(),
                    value: &load_balancers_binding,
                },
                register_interface::ObjectField {
                    name: "onDemandAllocationStrategy".into(),
                    value: &on_demand_allocation_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "onDemandMaxTotalPrice".into(),
                    value: &on_demand_max_total_price_binding,
                },
                register_interface::ObjectField {
                    name: "onDemandTargetCapacity".into(),
                    value: &on_demand_target_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "replaceUnhealthyInstances".into(),
                    value: &replace_unhealthy_instances_binding,
                },
                register_interface::ObjectField {
                    name: "spotMaintenanceStrategies".into(),
                    value: &spot_maintenance_strategies_binding,
                },
                register_interface::ObjectField {
                    name: "spotPrice".into(),
                    value: &spot_price_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetCapacity".into(),
                    value: &target_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "targetCapacityUnitType".into(),
                    value: &target_capacity_unit_type_binding,
                },
                register_interface::ObjectField {
                    name: "targetGroupArns".into(),
                    value: &target_group_arns_binding,
                },
                register_interface::ObjectField {
                    name: "terminateInstancesOnDelete".into(),
                    value: &terminate_instances_on_delete_binding,
                },
                register_interface::ObjectField {
                    name: "terminateInstancesWithExpiration".into(),
                    value: &terminate_instances_with_expiration_binding,
                },
                register_interface::ObjectField {
                    name: "validFrom".into(),
                    value: &valid_from_binding,
                },
                register_interface::ObjectField {
                    name: "validUntil".into(),
                    value: &valid_until_binding,
                },
                register_interface::ObjectField {
                    name: "waitForFulfillment".into(),
                    value: &wait_for_fulfillment_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpotFleetRequestResult {
            allocation_strategy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocationStrategy"),
            ),
            client_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientToken"),
            ),
            context: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("context"),
            ),
            excess_capacity_termination_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excessCapacityTerminationPolicy"),
            ),
            fleet_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fleetType"),
            ),
            iam_fleet_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamFleetRole"),
            ),
            instance_interruption_behaviour: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceInterruptionBehaviour"),
            ),
            instance_pools_to_use_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instancePoolsToUseCount"),
            ),
            launch_specifications: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("launchSpecifications"),
            ),
            launch_template_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("launchTemplateConfigs"),
            ),
            load_balancers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancers"),
            ),
            on_demand_allocation_strategy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("onDemandAllocationStrategy"),
            ),
            on_demand_max_total_price: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("onDemandMaxTotalPrice"),
            ),
            on_demand_target_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("onDemandTargetCapacity"),
            ),
            replace_unhealthy_instances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replaceUnhealthyInstances"),
            ),
            spot_maintenance_strategies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("spotMaintenanceStrategies"),
            ),
            spot_price: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("spotPrice"),
            ),
            spot_request_state: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("spotRequestState"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetCapacity"),
            ),
            target_capacity_unit_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetCapacityUnitType"),
            ),
            target_group_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetGroupArns"),
            ),
            terminate_instances_on_delete: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("terminateInstancesOnDelete"),
            ),
            terminate_instances_with_expiration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("terminateInstancesWithExpiration"),
            ),
            valid_from: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("validFrom"),
            ),
            valid_until: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("validUntil"),
            ),
            wait_for_fulfillment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("waitForFulfillment"),
            ),
        }
    }
}
