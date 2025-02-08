#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterNodePoolNodeConfigGuestAcceleratorGpuSharingConfig {
    /// The type of GPU sharing strategy to enable on the GPU node. Possible values are described in the API package (https://pkg.go.dev/google.golang.org/api/container/v1#GPUSharingConfig)
    #[builder(into)]
    #[serde(rename = "gpuSharingStrategy")]
    pub r#gpu_sharing_strategy: Box<String>,
    /// The maximum number of containers that can share a GPU.
    #[builder(into)]
    #[serde(rename = "maxSharedClientsPerGpu")]
    pub r#max_shared_clients_per_gpu: Box<i32>,
}
