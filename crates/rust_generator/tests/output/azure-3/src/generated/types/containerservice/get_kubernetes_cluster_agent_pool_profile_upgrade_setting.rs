#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterAgentPoolProfileUpgradeSetting {
    /// The amount of time in minutes to wait on eviction of pods and graceful termination per node. This eviction wait time honors waiting on pod disruption budgets. If this time is exceeded, the upgrade fails.
    #[builder(into)]
    #[serde(rename = "drainTimeoutInMinutes")]
    pub r#drain_timeout_in_minutes: Box<i32>,
    /// The maximum number or percentage of nodes that will be added to the Node Pool size during an upgrade.
    #[builder(into)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: Box<String>,
    /// The amount of time in minutes to wait after draining a node and before reimaging it and moving on to next node.
    #[builder(into)]
    #[serde(rename = "nodeSoakDurationInMinutes")]
    pub r#node_soak_duration_in_minutes: Box<i32>,
}
