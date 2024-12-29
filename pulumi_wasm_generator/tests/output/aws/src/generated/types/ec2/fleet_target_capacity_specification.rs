#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetTargetCapacitySpecification {
    /// Default target capacity type. Valid values: `on-demand`, `spot`.
    #[builder(into)]
    #[serde(rename = "defaultTargetCapacityType")]
    pub r#default_target_capacity_type: Box<String>,
    /// The number of On-Demand units to request.
    #[builder(into, default)]
    #[serde(rename = "onDemandTargetCapacity")]
    pub r#on_demand_target_capacity: Box<Option<i32>>,
    /// The number of Spot units to request.
    #[builder(into, default)]
    #[serde(rename = "spotTargetCapacity")]
    pub r#spot_target_capacity: Box<Option<i32>>,
    /// The unit for the target capacity.
    /// If you specify `target_capacity_unit_type`, `instance_requirements` must be specified.
    #[builder(into, default)]
    #[serde(rename = "targetCapacityUnitType")]
    pub r#target_capacity_unit_type: Box<Option<String>>,
    /// The number of units to request, filled using `default_target_capacity_type`.
    #[builder(into)]
    #[serde(rename = "totalTargetCapacity")]
    pub r#total_target_capacity: Box<i32>,
}
