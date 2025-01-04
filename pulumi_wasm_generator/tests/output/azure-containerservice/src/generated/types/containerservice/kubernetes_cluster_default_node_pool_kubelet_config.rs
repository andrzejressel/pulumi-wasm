#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterDefaultNodePoolKubeletConfig {
    /// Specifies the allow list of unsafe sysctls command or patterns (ending in `*`).
    #[builder(into, default)]
    #[serde(rename = "allowedUnsafeSysctls")]
    pub r#allowed_unsafe_sysctls: Box<Option<Vec<String>>>,
    /// Specifies the maximum number of container log files that can be present for a container. must be at least 2.
    #[builder(into, default)]
    #[serde(rename = "containerLogMaxLine")]
    pub r#container_log_max_line: Box<Option<i32>>,
    /// Specifies the maximum size (e.g. 10MB) of container log file before it is rotated.
    #[builder(into, default)]
    #[serde(rename = "containerLogMaxSizeMb")]
    pub r#container_log_max_size_mb: Box<Option<i32>>,
    /// Is CPU CFS quota enforcement for containers enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "cpuCfsQuotaEnabled")]
    pub r#cpu_cfs_quota_enabled: Box<Option<bool>>,
    /// Specifies the CPU CFS quota period value.
    #[builder(into, default)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: Box<Option<String>>,
    /// Specifies the CPU Manager policy to use. Possible values are `none` and `static`,.
    #[builder(into, default)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: Box<Option<String>>,
    /// Specifies the percent of disk usage above which image garbage collection is always run. Must be between `0` and `100`.
    #[builder(into, default)]
    #[serde(rename = "imageGcHighThreshold")]
    pub r#image_gc_high_threshold: Box<Option<i32>>,
    /// Specifies the percent of disk usage lower than which image garbage collection is never run. Must be between `0` and `100`.
    #[builder(into, default)]
    #[serde(rename = "imageGcLowThreshold")]
    pub r#image_gc_low_threshold: Box<Option<i32>>,
    /// Specifies the maximum number of processes per pod.
    #[builder(into, default)]
    #[serde(rename = "podMaxPid")]
    pub r#pod_max_pid: Box<Option<i32>>,
    /// Specifies the Topology Manager policy to use. Possible values are `none`, `best-effort`, `restricted` or `single-numa-node`.
    #[builder(into, default)]
    #[serde(rename = "topologyManagerPolicy")]
    pub r#topology_manager_policy: Box<Option<String>>,
}
