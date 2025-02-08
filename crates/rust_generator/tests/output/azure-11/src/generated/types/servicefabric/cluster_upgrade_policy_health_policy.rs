#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterUpgradePolicyHealthPolicy {
    /// Specifies the maximum tolerated percentage of applications that can have aggregated health state of error. If the upgrade exceeds this percentage, the cluster is unhealthy. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "maxUnhealthyApplicationsPercent")]
    pub r#max_unhealthy_applications_percent: Box<Option<i32>>,
    /// Specifies the maximum tolerated percentage of nodes that can have aggregated health states of error. If an upgrade exceeds this percentage, the cluster is unhealthy. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "maxUnhealthyNodesPercent")]
    pub r#max_unhealthy_nodes_percent: Box<Option<i32>>,
}
