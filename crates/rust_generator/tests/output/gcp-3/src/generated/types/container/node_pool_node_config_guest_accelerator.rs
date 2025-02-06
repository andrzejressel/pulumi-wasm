#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodePoolNodeConfigGuestAccelerator {
    /// The number of the accelerator cards exposed to an instance.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// Configuration for auto installation of GPU driver.
    #[builder(into, default)]
    #[serde(rename = "gpuDriverInstallationConfig")]
    pub r#gpu_driver_installation_config: Box<Option<super::super::types::container::NodePoolNodeConfigGuestAcceleratorGpuDriverInstallationConfig>>,
    /// Size of partitions to create on the GPU. Valid values are described in the NVIDIA mig user guide (https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning)
    #[builder(into, default)]
    #[serde(rename = "gpuPartitionSize")]
    pub r#gpu_partition_size: Box<Option<String>>,
    /// Configuration for GPU sharing.
    #[builder(into, default)]
    #[serde(rename = "gpuSharingConfig")]
    pub r#gpu_sharing_config: Box<Option<super::super::types::container::NodePoolNodeConfigGuestAcceleratorGpuSharingConfig>>,
    /// The accelerator type resource name.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
