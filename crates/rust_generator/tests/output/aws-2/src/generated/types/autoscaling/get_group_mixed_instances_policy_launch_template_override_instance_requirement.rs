#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirement {
    #[builder(into)]
    #[serde(rename = "acceleratorCounts")]
    pub r#accelerator_counts: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementAcceleratorCount>>,
    /// List of accelerator manufacturer names.
    #[builder(into)]
    #[serde(rename = "acceleratorManufacturers")]
    pub r#accelerator_manufacturers: Box<Vec<String>>,
    /// List of accelerator names.
    #[builder(into)]
    #[serde(rename = "acceleratorNames")]
    pub r#accelerator_names: Box<Vec<String>>,
    /// List of objects describing the minimum and maximum total memory of the accelerators.
    #[builder(into)]
    #[serde(rename = "acceleratorTotalMemoryMibs")]
    pub r#accelerator_total_memory_mibs: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementAcceleratorTotalMemoryMib>>,
    /// List of accelerator types.
    #[builder(into)]
    #[serde(rename = "acceleratorTypes")]
    pub r#accelerator_types: Box<Vec<String>>,
    /// List of instance types to apply the specified attributes against.
    #[builder(into)]
    #[serde(rename = "allowedInstanceTypes")]
    pub r#allowed_instance_types: Box<Vec<String>>,
    /// Indicates whether bare metal instances are included, excluded, or required.
    #[builder(into)]
    #[serde(rename = "bareMetal")]
    pub r#bare_metal: Box<String>,
    /// List of objects describing the minimum and maximum baseline EBS bandwidth (Mbps).
    #[builder(into)]
    #[serde(rename = "baselineEbsBandwidthMbps")]
    pub r#baseline_ebs_bandwidth_mbps: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementBaselineEbsBandwidthMbp>>,
    /// Indicates whether burstable performance instance types are included, excluded, or required.
    #[builder(into)]
    #[serde(rename = "burstablePerformance")]
    pub r#burstable_performance: Box<String>,
    /// List of CPU manufacturer names.
    #[builder(into)]
    #[serde(rename = "cpuManufacturers")]
    pub r#cpu_manufacturers: Box<Vec<String>>,
    /// List of excluded instance types.
    #[builder(into)]
    #[serde(rename = "excludedInstanceTypes")]
    pub r#excluded_instance_types: Box<Vec<String>>,
    /// List of instance generation names.
    #[builder(into)]
    #[serde(rename = "instanceGenerations")]
    pub r#instance_generations: Box<Vec<String>>,
    /// Indicates whether instance types with instance store volumes are included, excluded, or required.
    #[builder(into)]
    #[serde(rename = "localStorage")]
    pub r#local_storage: Box<String>,
    /// List of local storage type names.
    #[builder(into)]
    #[serde(rename = "localStorageTypes")]
    pub r#local_storage_types: Box<Vec<String>>,
    /// Price protection threshold for Spot Instances.
    #[builder(into)]
    #[serde(rename = "maxSpotPriceAsPercentageOfOptimalOnDemandPrice")]
    pub r#max_spot_price_as_percentage_of_optimal_on_demand_price: Box<i32>,
    /// List of objects describing the minimum and maximum amount of memory (GiB) per vCPU.
    #[builder(into)]
    #[serde(rename = "memoryGibPerVcpus")]
    pub r#memory_gib_per_vcpus: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementMemoryGibPerVcpus>>,
    /// List of objects describing the minimum and maximum amount of memory (MiB).
    #[builder(into)]
    #[serde(rename = "memoryMibs")]
    pub r#memory_mibs: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementMemoryMib>>,
    /// List of objects describing the minimum and maximum amount of network bandwidth (Gbps).
    #[builder(into)]
    #[serde(rename = "networkBandwidthGbps")]
    pub r#network_bandwidth_gbps: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementNetworkBandwidthGbp>>,
    /// List of objects describing the minimum and maximum amount of network interfaces.
    #[builder(into)]
    #[serde(rename = "networkInterfaceCounts")]
    pub r#network_interface_counts: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementNetworkInterfaceCount>>,
    /// Price protection threshold for On-Demand Instances.
    #[builder(into)]
    #[serde(rename = "onDemandMaxPricePercentageOverLowestPrice")]
    pub r#on_demand_max_price_percentage_over_lowest_price: Box<i32>,
    /// Indicates whether instance types must support On-Demand Instance Hibernation.
    #[builder(into)]
    #[serde(rename = "requireHibernateSupport")]
    pub r#require_hibernate_support: Box<bool>,
    /// Price protection threshold for Spot Instances.
    #[builder(into)]
    #[serde(rename = "spotMaxPricePercentageOverLowestPrice")]
    pub r#spot_max_price_percentage_over_lowest_price: Box<i32>,
    /// List of objects describing the minimum and maximum total storage (GB).
    #[builder(into)]
    #[serde(rename = "totalLocalStorageGbs")]
    pub r#total_local_storage_gbs: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementTotalLocalStorageGb>>,
    /// List of objects describing the minimum and maximum number of vCPUs.
    #[builder(into)]
    #[serde(rename = "vcpuCounts")]
    pub r#vcpu_counts: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementVcpuCount>>,
}
