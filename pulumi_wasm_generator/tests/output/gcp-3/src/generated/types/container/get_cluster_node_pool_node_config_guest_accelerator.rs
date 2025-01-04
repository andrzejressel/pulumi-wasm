#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolNodeConfigGuestAccelerator {
    /// The number of the accelerator cards exposed to an instance.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// Configuration for auto installation of GPU driver.
    #[builder(into)]
    #[serde(rename = "gpuDriverInstallationConfigs")]
    pub r#gpu_driver_installation_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigGuestAcceleratorGpuDriverInstallationConfig>>,
    /// Size of partitions to create on the GPU. Valid values are described in the NVIDIA mig user guide (https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning)
    #[builder(into)]
    #[serde(rename = "gpuPartitionSize")]
    pub r#gpu_partition_size: Box<String>,
    /// Configuration for GPU sharing.
    #[builder(into)]
    #[serde(rename = "gpuSharingConfigs")]
    pub r#gpu_sharing_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigGuestAcceleratorGpuSharingConfig>>,
    /// The accelerator type resource name.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
