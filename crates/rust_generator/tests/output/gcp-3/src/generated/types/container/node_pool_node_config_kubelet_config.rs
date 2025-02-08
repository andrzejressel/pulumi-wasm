#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodePoolNodeConfigKubeletConfig {
    /// Enable CPU CFS quota enforcement for containers that specify CPU limits.
    #[builder(into, default)]
    #[serde(rename = "cpuCfsQuota")]
    pub r#cpu_cfs_quota: Box<Option<bool>>,
    /// Set the CPU CFS quota period value 'cpu.cfs_period_us'.
    #[builder(into, default)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: Box<Option<String>>,
    /// Control the CPU management policy on the node.
    #[builder(into, default)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: Box<Option<String>>,
    /// Controls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`.
    #[builder(into, default)]
    #[serde(rename = "insecureKubeletReadonlyPortEnabled")]
    pub r#insecure_kubelet_readonly_port_enabled: Box<Option<String>>,
    /// Controls the maximum number of processes allowed to run in a pod.
    #[builder(into, default)]
    #[serde(rename = "podPidsLimit")]
    pub r#pod_pids_limit: Box<Option<i32>>,
}
