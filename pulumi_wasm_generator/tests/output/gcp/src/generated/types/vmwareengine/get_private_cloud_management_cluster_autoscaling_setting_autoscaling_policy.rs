#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicy {
    #[builder(into)]
    #[serde(rename = "autoscalePolicyId")]
    pub r#autoscale_policy_id: Box<String>,
    /// Utilization thresholds pertaining to amount of consumed memory.
    #[builder(into)]
    #[serde(rename = "consumedMemoryThresholds")]
    pub r#consumed_memory_thresholds: Box<Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold>>,
    /// Utilization thresholds pertaining to CPU utilization.
    #[builder(into)]
    #[serde(rename = "cpuThresholds")]
    pub r#cpu_thresholds: Box<Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyCpuThreshold>>,
    /// The canonical identifier of the node type to add or remove.
    #[builder(into)]
    #[serde(rename = "nodeTypeId")]
    pub r#node_type_id: Box<String>,
    /// Number of nodes to add to a cluster during a scale-out operation.
    /// Must be divisible by 2 for stretched clusters.
    #[builder(into)]
    #[serde(rename = "scaleOutSize")]
    pub r#scale_out_size: Box<i32>,
    /// Utilization thresholds pertaining to amount of consumed storage.
    #[builder(into)]
    #[serde(rename = "storageThresholds")]
    pub r#storage_thresholds: Box<Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyStorageThreshold>>,
}
