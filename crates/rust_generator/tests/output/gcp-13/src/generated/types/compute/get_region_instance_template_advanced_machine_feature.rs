#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRegionInstanceTemplateAdvancedMachineFeature {
    /// Whether to enable nested virtualization or not.
    #[builder(into)]
    #[serde(rename = "enableNestedVirtualization")]
    pub r#enable_nested_virtualization: Box<bool>,
    /// Whether to enable UEFI networking or not.
    #[builder(into)]
    #[serde(rename = "enableUefiNetworking")]
    pub r#enable_uefi_networking: Box<bool>,
    /// The PMU is a hardware component within the CPU core that monitors how the processor runs code. Valid values for the level of PMU are "STANDARD", "ENHANCED", and "ARCHITECTURAL".
    #[builder(into)]
    #[serde(rename = "performanceMonitoringUnit")]
    pub r#performance_monitoring_unit: Box<String>,
    /// The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed.
    #[builder(into)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: Box<i32>,
    /// Turbo frequency mode to use for the instance. Currently supported modes is "ALL_CORE_MAX".
    #[builder(into)]
    #[serde(rename = "turboMode")]
    pub r#turbo_mode: Box<String>,
    /// The number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\'s nominal CPU count and the underlying platform\'s SMT width.
    #[builder(into)]
    #[serde(rename = "visibleCoreCount")]
    pub r#visible_core_count: Box<i32>,
}
