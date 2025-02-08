#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterCoreInstanceFleet {
    /// ID of the cluster.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Configuration block for instance fleet.
    #[builder(into, default)]
    #[serde(rename = "instanceTypeConfigs")]
    pub r#instance_type_configs: Box<Option<Vec<super::super::types::emr::ClusterCoreInstanceFleetInstanceTypeConfig>>>,
    /// Configuration block for launch specification.
    #[builder(into, default)]
    #[serde(rename = "launchSpecifications")]
    pub r#launch_specifications: Box<Option<super::super::types::emr::ClusterCoreInstanceFleetLaunchSpecifications>>,
    /// Friendly name given to the instance fleet.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "provisionedOnDemandCapacity")]
    pub r#provisioned_on_demand_capacity: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "provisionedSpotCapacity")]
    pub r#provisioned_spot_capacity: Box<Option<i32>>,
    /// The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision.
    #[builder(into, default)]
    #[serde(rename = "targetOnDemandCapacity")]
    pub r#target_on_demand_capacity: Box<Option<i32>>,
    /// Target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision.
    #[builder(into, default)]
    #[serde(rename = "targetSpotCapacity")]
    pub r#target_spot_capacity: Box<Option<i32>>,
}
