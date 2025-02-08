#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGroupMixedInstancesPolicyInstancesDistribution {
    /// Strategy used when launching on-demand instances.
    #[builder(into)]
    #[serde(rename = "onDemandAllocationStrategy")]
    pub r#on_demand_allocation_strategy: Box<String>,
    /// Absolute minimum amount of desired capacity that must be fulfilled by on-demand instances.
    #[builder(into)]
    #[serde(rename = "onDemandBaseCapacity")]
    pub r#on_demand_base_capacity: Box<i32>,
    #[builder(into)]
    #[serde(rename = "onDemandPercentageAboveBaseCapacity")]
    pub r#on_demand_percentage_above_base_capacity: Box<i32>,
    /// Strategy used when launching Spot instances.
    #[builder(into)]
    #[serde(rename = "spotAllocationStrategy")]
    pub r#spot_allocation_strategy: Box<String>,
    /// Number of Spot pools per availability zone to allocate capacity.
    #[builder(into)]
    #[serde(rename = "spotInstancePools")]
    pub r#spot_instance_pools: Box<i32>,
    /// Maximum price per unit hour that the user is willing to pay for the Spot instances.
    #[builder(into)]
    #[serde(rename = "spotMaxPrice")]
    pub r#spot_max_price: Box<String>,
}
