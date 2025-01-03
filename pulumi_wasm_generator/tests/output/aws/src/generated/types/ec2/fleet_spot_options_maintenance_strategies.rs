#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetSpotOptionsMaintenanceStrategies {
    /// Nested argument containing the capacity rebalance for your fleet request. Defined below.
    #[builder(into, default)]
    #[serde(rename = "capacityRebalance")]
    pub r#capacity_rebalance: Box<Option<super::super::types::ec2::FleetSpotOptionsMaintenanceStrategiesCapacityRebalance>>,
}
