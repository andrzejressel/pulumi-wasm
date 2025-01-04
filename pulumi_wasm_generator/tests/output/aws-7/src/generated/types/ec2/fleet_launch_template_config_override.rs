#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetLaunchTemplateConfigOverride {
    /// Availability Zone in which to launch the instances.
    #[builder(into, default)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<Option<String>>,
    /// Override the instance type in the Launch Template with instance types that satisfy the requirements.
    #[builder(into, default)]
    #[serde(rename = "instanceRequirements")]
    pub r#instance_requirements: Box<Option<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirements>>,
    /// Instance type.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// Maximum price per unit hour that you are willing to pay for a Spot Instance.
    #[builder(into, default)]
    #[serde(rename = "maxPrice")]
    pub r#max_price: Box<Option<String>>,
    /// Priority for the launch template override. If `on_demand_options` `allocation_strategy` is set to `prioritized`, EC2 Fleet uses priority to determine which launch template override to use first in fulfilling On-Demand capacity. The highest priority is launched first. The lower the number, the higher the priority. If no number is set, the launch template override has the lowest priority. Valid values are whole numbers starting at 0.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<f64>>,
    /// ID of the subnet in which to launch the instances.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
    /// Number of units provided by the specified instance type.
    #[builder(into, default)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Box<Option<f64>>,
}
