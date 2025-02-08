#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetSpotOptions {
    /// How to allocate the target capacity across the Spot pools. Valid values: `diversified`, `lowestPrice`, `capacity-optimized`, `capacity-optimized-prioritized` and `price-capacity-optimized`. Default: `lowestPrice`.
    #[builder(into, default)]
    #[serde(rename = "allocationStrategy")]
    pub r#allocation_strategy: Box<Option<String>>,
    /// Behavior when a Spot Instance is interrupted. Valid values: `hibernate`, `stop`, `terminate`. Default: `terminate`.
    #[builder(into, default)]
    #[serde(rename = "instanceInterruptionBehavior")]
    pub r#instance_interruption_behavior: Box<Option<String>>,
    /// Number of Spot pools across which to allocate your target Spot capacity. Valid only when Spot `allocation_strategy` is set to `lowestPrice`. Default: `1`.
    #[builder(into, default)]
    #[serde(rename = "instancePoolsToUseCount")]
    pub r#instance_pools_to_use_count: Box<Option<i32>>,
    /// Nested argument containing maintenance strategies for managing your Spot Instances that are at an elevated risk of being interrupted. Defined below.
    #[builder(into, default)]
    #[serde(rename = "maintenanceStrategies")]
    pub r#maintenance_strategies: Box<Option<super::super::types::ec2::FleetSpotOptionsMaintenanceStrategies>>,
}
