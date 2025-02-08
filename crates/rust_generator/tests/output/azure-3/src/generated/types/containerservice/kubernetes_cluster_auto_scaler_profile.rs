#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KubernetesClusterAutoScalerProfile {
    /// Detect similar node groups and balance the number of nodes between them. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "balanceSimilarNodeGroups")]
    pub r#balance_similar_node_groups: Box<Option<bool>>,
    /// Whether DaemonSet pods will be gracefully terminated from empty nodes. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "daemonsetEvictionForEmptyNodesEnabled")]
    pub r#daemonset_eviction_for_empty_nodes_enabled: Box<Option<bool>>,
    /// Whether DaemonSet pods will be gracefully terminated from non-empty nodes. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "daemonsetEvictionForOccupiedNodesEnabled")]
    pub r#daemonset_eviction_for_occupied_nodes_enabled: Box<Option<bool>>,
    /// Maximum number of empty nodes that can be deleted at the same time. Defaults to `10`.
    #[builder(into, default)]
    #[serde(rename = "emptyBulkDeleteMax")]
    pub r#empty_bulk_delete_max: Box<Option<String>>,
    /// Expander to use. Possible values are `least-waste`, `priority`, `most-pods` and `random`. Defaults to `random`.
    #[builder(into, default)]
    #[serde(rename = "expander")]
    pub r#expander: Box<Option<String>>,
    /// Whether DaemonSet pods will be ignored when calculating resource utilization for scale down. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "ignoreDaemonsetsUtilizationEnabled")]
    pub r#ignore_daemonsets_utilization_enabled: Box<Option<bool>>,
    /// Maximum number of seconds the cluster autoscaler waits for pod termination when trying to scale down a node. Defaults to `600`.
    #[builder(into, default)]
    #[serde(rename = "maxGracefulTerminationSec")]
    pub r#max_graceful_termination_sec: Box<Option<String>>,
    /// Maximum time the autoscaler waits for a node to be provisioned. Defaults to `15m`.
    #[builder(into, default)]
    #[serde(rename = "maxNodeProvisioningTime")]
    pub r#max_node_provisioning_time: Box<Option<String>>,
    /// Maximum Number of allowed unready nodes. Defaults to `3`.
    #[builder(into, default)]
    #[serde(rename = "maxUnreadyNodes")]
    pub r#max_unready_nodes: Box<Option<i32>>,
    /// Maximum percentage of unready nodes the cluster autoscaler will stop if the percentage is exceeded. Defaults to `45`.
    #[builder(into, default)]
    #[serde(rename = "maxUnreadyPercentage")]
    pub r#max_unready_percentage: Box<Option<f64>>,
    /// For scenarios like burst/batch scale where you don't want CA to act before the kubernetes scheduler could schedule all the pods, you can tell CA to ignore unscheduled pods before they're a certain age. Defaults to `10s`.
    #[builder(into, default)]
    #[serde(rename = "newPodScaleUpDelay")]
    pub r#new_pod_scale_up_delay: Box<Option<String>>,
    /// How long after the scale up of AKS nodes the scale down evaluation resumes. Defaults to `10m`.
    #[builder(into, default)]
    #[serde(rename = "scaleDownDelayAfterAdd")]
    pub r#scale_down_delay_after_add: Box<Option<String>>,
    /// How long after node deletion that scale down evaluation resumes. Defaults to the value used for `scan_interval`.
    #[builder(into, default)]
    #[serde(rename = "scaleDownDelayAfterDelete")]
    pub r#scale_down_delay_after_delete: Box<Option<String>>,
    /// How long after scale down failure that scale down evaluation resumes. Defaults to `3m`.
    #[builder(into, default)]
    #[serde(rename = "scaleDownDelayAfterFailure")]
    pub r#scale_down_delay_after_failure: Box<Option<String>>,
    /// How long a node should be unneeded before it is eligible for scale down. Defaults to `10m`.
    #[builder(into, default)]
    #[serde(rename = "scaleDownUnneeded")]
    pub r#scale_down_unneeded: Box<Option<String>>,
    /// How long an unready node should be unneeded before it is eligible for scale down. Defaults to `20m`.
    #[builder(into, default)]
    #[serde(rename = "scaleDownUnready")]
    pub r#scale_down_unready: Box<Option<String>>,
    /// Node utilization level, defined as sum of requested resources divided by capacity, below which a node can be considered for scale down. Defaults to `0.5`.
    #[builder(into, default)]
    #[serde(rename = "scaleDownUtilizationThreshold")]
    pub r#scale_down_utilization_threshold: Box<Option<String>>,
    /// How often the AKS Cluster should be re-evaluated for scale up/down. Defaults to `10s`.
    #[builder(into, default)]
    #[serde(rename = "scanInterval")]
    pub r#scan_interval: Box<Option<String>>,
    /// If `true` cluster autoscaler will never delete nodes with pods with local storage, for example, EmptyDir or HostPath. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "skipNodesWithLocalStorage")]
    pub r#skip_nodes_with_local_storage: Box<Option<bool>>,
    /// If `true` cluster autoscaler will never delete nodes with pods from kube-system (except for DaemonSet or mirror pods). Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "skipNodesWithSystemPods")]
    pub r#skip_nodes_with_system_pods: Box<Option<bool>>,
}
