#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicy {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "autoscalePolicyId")]
    pub r#autoscale_policy_id: Box<String>,
    /// Utilization thresholds pertaining to amount of consumed memory.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "consumedMemoryThresholds")]
    pub r#consumed_memory_thresholds: Box<Option<super::super::types::vmwareengine::PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholds>>,
    /// Utilization thresholds pertaining to CPU utilization.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cpuThresholds")]
    pub r#cpu_thresholds: Box<Option<super::super::types::vmwareengine::PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholds>>,
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
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "storageThresholds")]
    pub r#storage_thresholds: Box<Option<super::super::types::vmwareengine::PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds>>,
}
