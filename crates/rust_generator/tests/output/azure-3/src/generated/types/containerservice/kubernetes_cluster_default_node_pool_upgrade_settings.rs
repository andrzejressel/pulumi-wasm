#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterDefaultNodePoolUpgradeSettings {
    /// The amount of time in minutes to wait on eviction of pods and graceful termination per node. This eviction wait time honors pod disruption budgets for upgrades. If this time is exceeded, the upgrade fails. Unsetting this after configuring it will force a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "drainTimeoutInMinutes")]
    pub r#drain_timeout_in_minutes: Box<Option<i32>>,
    /// The maximum number or percentage of nodes which will be added to the Node Pool size during an upgrade.
    /// 
    /// > **Note:** If a percentage is provided, the number of surge nodes is calculated from the `node_count` value on the current cluster. Node surge can allow a cluster to have more nodes than `max_count` during an upgrade. Ensure that your cluster has enough [IP space](https://docs.microsoft.com/azure/aks/upgrade-cluster#customize-node-surge-upgrade) during an upgrade.
    #[builder(into)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: Box<String>,
    /// The amount of time in minutes to wait after draining a node and before reimaging and moving on to next node. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "nodeSoakDurationInMinutes")]
    pub r#node_soak_duration_in_minutes: Box<Option<i32>>,
}
