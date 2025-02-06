#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodeConfigKubeletConfig {
    /// If true, enables CPU CFS quota enforcement for
    /// containers that specify CPU limits.
    #[builder(into, default)]
    #[serde(rename = "cpuCfsQuota")]
    pub r#cpu_cfs_quota: Box<Option<bool>>,
    /// The CPU CFS quota period value. Specified
    /// as a sequence of decimal numbers, each with optional fraction and a unit suffix,
    /// such as `"300ms"`. Valid time units are "ns", "us" (or "µs"), "ms", "s", "m",
    /// "h". The value must be a positive duration.
    #[builder(into, default)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: Box<Option<String>>,
    /// The CPU management policy on the node. See
    /// [K8S CPU Management Policies](https://kubernetes.io/docs/tasks/administer-cluster/cpu-management-policies/).
    /// One of `"none"` or `"static"`. If unset (or set to the empty string `""`), the API will treat the field as if set to "none".
    /// Prior to the 6.4.0 this field was marked as required. The workaround for the required field
    /// is setting the empty string `""`, which will function identically to not setting this field.
    #[builder(into, default)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: Box<Option<String>>,
    /// Controls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`.
    #[builder(into, default)]
    #[serde(rename = "insecureKubeletReadonlyPortEnabled")]
    pub r#insecure_kubelet_readonly_port_enabled: Box<Option<String>>,
    /// Controls the maximum number of processes allowed to run in a pod. The value must be greater than or equal to 1024 and less than 4194304.
    #[builder(into, default)]
    #[serde(rename = "podPidsLimit")]
    pub r#pod_pids_limit: Box<Option<i32>>,
}
