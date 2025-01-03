#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetLaunchTemplateConfigOverrideInstanceRequirements {
    /// Block describing the minimum and maximum number of accelerators (GPUs, FPGAs, or AWS Inferentia chips). Default is no minimum or maximum limits.
    #[builder(into, default)]
    #[serde(rename = "acceleratorCount")]
    pub r#accelerator_count: Box<Option<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirementsAcceleratorCount>>,
    /// List of accelerator manufacturer names. Default is any manufacturer.
    #[builder(into, default)]
    #[serde(rename = "acceleratorManufacturers")]
    pub r#accelerator_manufacturers: Box<Option<Vec<String>>>,
    /// List of accelerator names. Default is any acclerator.
    #[builder(into, default)]
    #[serde(rename = "acceleratorNames")]
    pub r#accelerator_names: Box<Option<Vec<String>>>,
    /// Block describing the minimum and maximum total memory of the accelerators. Default is no minimum or maximum.
    #[builder(into, default)]
    #[serde(rename = "acceleratorTotalMemoryMib")]
    pub r#accelerator_total_memory_mib: Box<Option<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirementsAcceleratorTotalMemoryMib>>,
    /// The accelerator types that must be on the instance type. Default is any accelerator type.
    #[builder(into, default)]
    #[serde(rename = "acceleratorTypes")]
    pub r#accelerator_types: Box<Option<Vec<String>>>,
    /// The instance types to apply your specified attributes against. All other instance types are ignored, even if they match your specified attributes. You can use strings with one or more wild cards,represented by an asterisk (\*). The following are examples: `c5*`, `m5a.*`, `r*`, `*3*`. For example, if you specify `c5*`, you are excluding the entire C5 instance family, which includes all C5a and C5n instance types. If you specify `m5a.*`, you are excluding all the M5a instance types, but not the M5n instance types. Maximum of 400 entries in the list; each entry is limited to 30 characters. Default is no excluded instance types. Default is any instance type.
    /// 
    /// If you specify `AllowedInstanceTypes`, you can't specify `ExcludedInstanceTypes`.
    #[builder(into, default)]
    #[serde(rename = "allowedInstanceTypes")]
    pub r#allowed_instance_types: Box<Option<Vec<String>>>,
    /// Indicate whether bare metal instace types should be `included`, `excluded`, or `required`. Default is `excluded`.
    #[builder(into, default)]
    #[serde(rename = "bareMetal")]
    pub r#bare_metal: Box<Option<String>>,
    /// Block describing the minimum and maximum baseline EBS bandwidth, in Mbps. Default is no minimum or maximum.
    #[builder(into, default)]
    #[serde(rename = "baselineEbsBandwidthMbps")]
    pub r#baseline_ebs_bandwidth_mbps: Box<Option<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirementsBaselineEbsBandwidthMbps>>,
    /// Indicates whether burstable performance T instance types are `included`, `excluded`, or `required`. Default is `excluded`.
    #[builder(into, default)]
    #[serde(rename = "burstablePerformance")]
    pub r#burstable_performance: Box<Option<String>>,
    /// The CPU manufacturers to include. Default is any manufacturer.
    /// > **NOTE:** Don't confuse the CPU hardware manufacturer with the CPU hardware architecture. Instances will be launched with a compatible CPU architecture based on the Amazon Machine Image (AMI) that you specify in your launch template.
    #[builder(into, default)]
    #[serde(rename = "cpuManufacturers")]
    pub r#cpu_manufacturers: Box<Option<Vec<String>>>,
    /// The instance types to exclude. You can use strings with one or more wild cards, represented by an asterisk (\*). The following are examples: `c5*`, `m5a.*`, `r*`, `*3*`. For example, if you specify `c5*`, you are excluding the entire C5 instance family, which includes all C5a and C5n instance types. If you specify `m5a.*`, you are excluding all the M5a instance types, but not the M5n instance types. Maximum of 400 entries in the list; each entry is limited to 30 characters. Default is no excluded instance types.
    /// 
    /// If you specify `AllowedInstanceTypes`, you can't specify `ExcludedInstanceTypes`.
    #[builder(into, default)]
    #[serde(rename = "excludedInstanceTypes")]
    pub r#excluded_instance_types: Box<Option<Vec<String>>>,
    /// Indicates whether current or previous generation instance types are included. The current generation instance types are recommended for use. Valid values are `current` and `previous`. Default is `current` and `previous` generation instance types.
    #[builder(into, default)]
    #[serde(rename = "instanceGenerations")]
    pub r#instance_generations: Box<Option<Vec<String>>>,
    /// Indicate whether instance types with local storage volumes are `included`, `excluded`, or `required`. Default is `included`.
    #[builder(into, default)]
    #[serde(rename = "localStorage")]
    pub r#local_storage: Box<Option<String>>,
    /// List of local storage type names. Valid values are `hdd` and `ssd`. Default any storage type.
    #[builder(into, default)]
    #[serde(rename = "localStorageTypes")]
    pub r#local_storage_types: Box<Option<Vec<String>>>,
    /// The price protection threshold for Spot Instances. This is the maximum you’ll pay for a Spot Instance, expressed as a percentage higher than the cheapest M, C, or R instance type with your specified attributes. When Amazon EC2 Auto Scaling selects instance types with your attributes, we will exclude instance types whose price is higher than your threshold. The parameter accepts an integer, which Amazon EC2 Auto Scaling interprets as a percentage. To turn off price protection, specify a high value, such as 999999. Conflicts with `spot_max_price_percentage_over_lowest_price`
    #[builder(into, default)]
    #[serde(rename = "maxSpotPriceAsPercentageOfOptimalOnDemandPrice")]
    pub r#max_spot_price_as_percentage_of_optimal_on_demand_price: Box<Option<i32>>,
    /// Block describing the minimum and maximum amount of memory (GiB) per vCPU. Default is no minimum or maximum.
    #[builder(into, default)]
    #[serde(rename = "memoryGibPerVcpu")]
    pub r#memory_gib_per_vcpu: Box<Option<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirementsMemoryGibPerVcpu>>,
    /// The minimum and maximum amount of memory per vCPU, in GiB. Default is no minimum or maximum limits.
    #[builder(into)]
    #[serde(rename = "memoryMib")]
    pub r#memory_mib: Box<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirementsMemoryMib>,
    /// The minimum and maximum amount of network bandwidth, in gigabits per second (Gbps). Default is No minimum or maximum.
    #[builder(into, default)]
    #[serde(rename = "networkBandwidthGbps")]
    pub r#network_bandwidth_gbps: Box<Option<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirementsNetworkBandwidthGbps>>,
    /// Block describing the minimum and maximum number of network interfaces. Default is no minimum or maximum.
    #[builder(into, default)]
    #[serde(rename = "networkInterfaceCount")]
    pub r#network_interface_count: Box<Option<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirementsNetworkInterfaceCount>>,
    /// The price protection threshold for On-Demand Instances. This is the maximum you’ll pay for an On-Demand Instance, expressed as a percentage higher than the cheapest M, C, or R instance type with your specified attributes. When Amazon EC2 Auto Scaling selects instance types with your attributes, we will exclude instance types whose price is higher than your threshold. The parameter accepts an integer, which Amazon EC2 Auto Scaling interprets as a percentage. To turn off price protection, specify a high value, such as 999999. Default is 20.
    /// 
    /// If you set `target_capacity_unit_type` to `vcpu` or `memory-mib`, the price protection threshold is applied based on the per-vCPU or per-memory price instead of the per-instance price.
    #[builder(into, default)]
    #[serde(rename = "onDemandMaxPricePercentageOverLowestPrice")]
    pub r#on_demand_max_price_percentage_over_lowest_price: Box<Option<i32>>,
    /// Indicate whether instance types must support On-Demand Instance Hibernation, either `true` or `false`. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "requireHibernateSupport")]
    pub r#require_hibernate_support: Box<Option<bool>>,
    /// The price protection threshold for Spot Instances. This is the maximum you’ll pay for a Spot Instance, expressed as a percentage higher than the cheapest M, C, or R instance type with your specified attributes. When Amazon EC2 Auto Scaling selects instance types with your attributes, we will exclude instance types whose price is higher than your threshold. The parameter accepts an integer, which Amazon EC2 Auto Scaling interprets as a percentage. To turn off price protection, specify a high value, such as 999999. Default is 100. Conflicts with `max_spot_price_as_percentage_of_optimal_on_demand_price`
    /// 
    /// If you set DesiredCapacityType to vcpu or memory-mib, the price protection threshold is applied based on the per vCPU or per memory price instead of the per instance price.
    #[builder(into, default)]
    #[serde(rename = "spotMaxPricePercentageOverLowestPrice")]
    pub r#spot_max_price_percentage_over_lowest_price: Box<Option<i32>>,
    /// Block describing the minimum and maximum total local storage (GB). Default is no minimum or maximum.
    #[builder(into, default)]
    #[serde(rename = "totalLocalStorageGb")]
    pub r#total_local_storage_gb: Box<Option<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirementsTotalLocalStorageGb>>,
    /// Block describing the minimum and maximum number of vCPUs. Default is no maximum.
    #[builder(into)]
    #[serde(rename = "vcpuCount")]
    pub r#vcpu_count: Box<super::super::types::ec2::FleetLaunchTemplateConfigOverrideInstanceRequirementsVcpuCount>,
}
