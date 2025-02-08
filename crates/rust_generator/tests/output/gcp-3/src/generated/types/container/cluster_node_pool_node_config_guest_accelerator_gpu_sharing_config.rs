#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodePoolNodeConfigGuestAcceleratorGpuSharingConfig {
    /// The type of GPU sharing strategy to enable on the GPU node.
    /// Accepted values are:
    /// * `"TIME_SHARING"`: Allow multiple containers to have [time-shared](https://cloud.google.com/kubernetes-engine/docs/concepts/timesharing-gpus) access to a single GPU device.
    /// * `"MPS"`: Enable co-operative multi-process CUDA workloads to run concurrently on a single GPU device with [MPS](https://cloud.google.com/kubernetes-engine/docs/how-to/nvidia-mps-gpus)
    #[builder(into)]
    #[serde(rename = "gpuSharingStrategy")]
    pub r#gpu_sharing_strategy: Box<String>,
    /// The maximum number of containers that can share a GPU.
    #[builder(into)]
    #[serde(rename = "maxSharedClientsPerGpu")]
    pub r#max_shared_clients_per_gpu: Box<i32>,
}
