/// Provides an Elastic MapReduce Cluster Instance Fleet configuration.
/// See [Amazon Elastic MapReduce Documentation](https://aws.amazon.com/documentation/emr/) for more information.
///
/// > **NOTE:** At this time, Instance Fleets cannot be destroyed through the API nor
/// web interface. Instance Fleets are destroyed when the EMR Cluster is destroyed.
/// the provider will resize any Instance Fleet to zero when destroying the resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let task = instance_fleet::create(
///         "task",
///         InstanceFleetArgs::builder()
///             .cluster_id("${cluster.id}")
///             .instance_type_configs(
///                 vec![
///                     InstanceFleetInstanceTypeConfig::builder()
///                     .bidPriceAsPercentageOfOnDemandPrice(100)
///                     .ebsConfigs(vec![InstanceFleetInstanceTypeConfigEbsConfig::builder()
///                     .size(100). type ("gp2").volumesPerInstance(1).build_struct(),])
///                     .instanceType("m4.xlarge").weightedCapacity(1).build_struct(),
///                     InstanceFleetInstanceTypeConfig::builder()
///                     .bidPriceAsPercentageOfOnDemandPrice(100)
///                     .ebsConfigs(vec![InstanceFleetInstanceTypeConfigEbsConfig::builder()
///                     .size(100). type ("gp2").volumesPerInstance(1).build_struct(),])
///                     .instanceType("m4.2xlarge").weightedCapacity(2).build_struct(),
///                 ],
///             )
///             .launch_specifications(
///                 InstanceFleetLaunchSpecifications::builder()
///                     .spotSpecifications(
///                         vec![
///                             InstanceFleetLaunchSpecificationsSpotSpecification::builder()
///                             .allocationStrategy("capacity-optimized")
///                             .blockDurationMinutes(0).timeoutAction("TERMINATE_CLUSTER")
///                             .timeoutDurationMinutes(10).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("task fleet")
///             .target_on_demand_capacity(1)
///             .target_spot_capacity(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR Instance Fleet using the EMR Cluster identifier and Instance Fleet identifier separated by a forward slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:emr/instanceFleet:InstanceFleet example j-123456ABCDEF/if-15EK4O09RZLNR
/// ```
pub mod instance_fleet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceFleetArgs {
        /// ID of the EMR Cluster to attach to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configuration block for instance fleet
        #[builder(into, default)]
        pub instance_type_configs: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::emr::InstanceFleetInstanceTypeConfig>>,
        >,
        /// Configuration block for launch specification
        #[builder(into, default)]
        pub launch_specifications: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::emr::InstanceFleetLaunchSpecifications>,
        >,
        /// Friendly name given to the instance fleet.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision.
        #[builder(into, default)]
        pub target_on_demand_capacity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision.
        #[builder(into, default)]
        pub target_spot_capacity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct InstanceFleetResult {
        /// ID of the EMR Cluster to attach to. Changing this forces a new resource to be created.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Configuration block for instance fleet
        pub instance_type_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::emr::InstanceFleetInstanceTypeConfig>>,
        >,
        /// Configuration block for launch specification
        pub launch_specifications: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::InstanceFleetLaunchSpecifications>,
        >,
        /// Friendly name given to the instance fleet.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of On-Demand units that have been provisioned for the instance
        /// fleet to fulfill TargetOnDemandCapacity. This provisioned capacity might be less than or greater than TargetOnDemandCapacity.
        pub provisioned_on_demand_capacity: pulumi_wasm_rust::Output<i32>,
        /// The number of Spot units that have been provisioned for this instance fleet
        /// to fulfill TargetSpotCapacity. This provisioned capacity might be less than or greater than TargetSpotCapacity.
        pub provisioned_spot_capacity: pulumi_wasm_rust::Output<i32>,
        /// The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision.
        pub target_on_demand_capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// The target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision.
        pub target_spot_capacity: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceFleetArgs,
    ) -> InstanceFleetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_output(context).get_inner();
        let instance_type_configs_binding = args
            .instance_type_configs
            .get_output(context)
            .get_inner();
        let launch_specifications_binding = args
            .launch_specifications
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let target_on_demand_capacity_binding = args
            .target_on_demand_capacity
            .get_output(context)
            .get_inner();
        let target_spot_capacity_binding = args
            .target_spot_capacity
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emr/instanceFleet:InstanceFleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceTypeConfigs".into(),
                    value: &instance_type_configs_binding,
                },
                register_interface::ObjectField {
                    name: "launchSpecifications".into(),
                    value: &launch_specifications_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "targetOnDemandCapacity".into(),
                    value: &target_on_demand_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "targetSpotCapacity".into(),
                    value: &target_spot_capacity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "instanceTypeConfigs".into(),
                },
                register_interface::ResultField {
                    name: "launchSpecifications".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "provisionedOnDemandCapacity".into(),
                },
                register_interface::ResultField {
                    name: "provisionedSpotCapacity".into(),
                },
                register_interface::ResultField {
                    name: "targetOnDemandCapacity".into(),
                },
                register_interface::ResultField {
                    name: "targetSpotCapacity".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceFleetResult {
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            instance_type_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceTypeConfigs").unwrap(),
            ),
            launch_specifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchSpecifications").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            provisioned_on_demand_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisionedOnDemandCapacity").unwrap(),
            ),
            provisioned_spot_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisionedSpotCapacity").unwrap(),
            ),
            target_on_demand_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetOnDemandCapacity").unwrap(),
            ),
            target_spot_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetSpotCapacity").unwrap(),
            ),
        }
    }
}
