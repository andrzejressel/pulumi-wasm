#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterNodeConfigGuestAccelerator {
    /// The number of the guest accelerator cards exposed to this instance.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// Configuration for auto installation of GPU driver. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gpuDriverInstallationConfig")]
    pub r#gpu_driver_installation_config: Box<Option<super::super::types::container::ClusterNodeConfigGuestAcceleratorGpuDriverInstallationConfig>>,
    /// Size of partitions to create on the GPU. Valid values are described in the NVIDIA mig [user guide](https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning).
    #[builder(into, default)]
    #[serde(rename = "gpuPartitionSize")]
    pub r#gpu_partition_size: Box<Option<String>>,
    /// Configuration for GPU sharing. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gpuSharingConfig")]
    pub r#gpu_sharing_config: Box<Option<super::super::types::container::ClusterNodeConfigGuestAcceleratorGpuSharingConfig>>,
    /// The accelerator type resource to expose to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
