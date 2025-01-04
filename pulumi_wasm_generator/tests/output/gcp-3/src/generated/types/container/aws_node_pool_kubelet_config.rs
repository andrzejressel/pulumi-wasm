#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsNodePoolKubeletConfig {
    /// Whether or not to enable CPU CFS quota. Defaults to true.
    #[builder(into, default)]
    #[serde(rename = "cpuCfsQuota")]
    pub r#cpu_cfs_quota: Box<Option<bool>>,
    /// Optional. The CPU CFS quota period to use for the node. Defaults to "100ms".
    #[builder(into, default)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: Box<Option<String>>,
    /// The CpuManagerPolicy to use for the node. Defaults to "none".
    #[builder(into, default)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: Box<Option<String>>,
    /// Optional. The maximum number of PIDs in each pod running on the node. The limit scales automatically based on underlying machine size if left unset.
    #[builder(into, default)]
    #[serde(rename = "podPidsLimit")]
    pub r#pod_pids_limit: Box<Option<i32>>,
}
