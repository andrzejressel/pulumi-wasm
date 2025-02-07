#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedScalingPolicyComputeLimit {
    /// The upper boundary of EC2 units. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. Managed scaling activities are not allowed beyond this boundary. The limit only applies to the core and task nodes. The master node cannot be scaled after initial configuration.
    #[builder(into)]
    #[serde(rename = "maximumCapacityUnits")]
    pub r#maximum_capacity_units: Box<i32>,
    /// The upper boundary of EC2 units for core node type in a cluster. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. The core units are not allowed to scale beyond this boundary. The parameter is used to split capacity allocation between core and task nodes.
    #[builder(into, default)]
    #[serde(rename = "maximumCoreCapacityUnits")]
    pub r#maximum_core_capacity_units: Box<Option<i32>>,
    /// The upper boundary of On-Demand EC2 units. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. The On-Demand units are not allowed to scale beyond this boundary. The parameter is used to split capacity allocation between On-Demand and Spot instances.
    #[builder(into, default)]
    #[serde(rename = "maximumOndemandCapacityUnits")]
    pub r#maximum_ondemand_capacity_units: Box<Option<i32>>,
    /// The lower boundary of EC2 units. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. Managed scaling activities are not allowed beyond this boundary. The limit only applies to the core and task nodes. The master node cannot be scaled after initial configuration.
    #[builder(into)]
    #[serde(rename = "minimumCapacityUnits")]
    pub r#minimum_capacity_units: Box<i32>,
    /// The unit type used for specifying a managed scaling policy. Valid Values: `InstanceFleetUnits` | `Instances` | `VCPU`
    #[builder(into)]
    #[serde(rename = "unitType")]
    pub r#unit_type: Box<String>,
}
