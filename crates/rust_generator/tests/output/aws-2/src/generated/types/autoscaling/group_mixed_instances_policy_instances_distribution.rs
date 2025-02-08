#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupMixedInstancesPolicyInstancesDistribution {
    /// Strategy to use when launching on-demand instances. Valid values: `prioritized`, `lowest-price`. Default: `prioritized`.
    #[builder(into, default)]
    #[serde(rename = "onDemandAllocationStrategy")]
    pub r#on_demand_allocation_strategy: Box<Option<String>>,
    /// Absolute minimum amount of desired capacity that must be fulfilled by on-demand instances. Default: `0`.
    #[builder(into, default)]
    #[serde(rename = "onDemandBaseCapacity")]
    pub r#on_demand_base_capacity: Box<Option<i32>>,
    /// Percentage split between on-demand and Spot instances above the base on-demand capacity. Default: `100`.
    #[builder(into, default)]
    #[serde(rename = "onDemandPercentageAboveBaseCapacity")]
    pub r#on_demand_percentage_above_base_capacity: Box<Option<i32>>,
    /// How to allocate capacity across the Spot pools. Valid values: `lowest-price`, `capacity-optimized`, `capacity-optimized-prioritized`, and `price-capacity-optimized`. Default: `lowest-price`.
    #[builder(into, default)]
    #[serde(rename = "spotAllocationStrategy")]
    pub r#spot_allocation_strategy: Box<Option<String>>,
    /// Number of Spot pools per availability zone to allocate capacity. EC2 Auto Scaling selects the cheapest Spot pools and evenly allocates Spot capacity across the number of Spot pools that you specify. Only available with `spot_allocation_strategy` set to `lowest-price`. Otherwise it must be set to `0`, if it has been defined before. Default: `2`.
    #[builder(into, default)]
    #[serde(rename = "spotInstancePools")]
    pub r#spot_instance_pools: Box<Option<i32>>,
    /// Maximum price per unit hour that the user is willing to pay for the Spot instances. Default: an empty string which means the on-demand price.
    #[builder(into, default)]
    #[serde(rename = "spotMaxPrice")]
    pub r#spot_max_price: Box<Option<String>>,
}
