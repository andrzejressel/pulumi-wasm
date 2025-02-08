#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterUpgradePolicyDeltaHealthPolicy {
    /// Specifies the maximum tolerated percentage of delta unhealthy applications that can have aggregated health states of error. If the current unhealthy applications do not respect the percentage relative to the state at the beginning of the upgrade, the cluster is unhealthy. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "maxDeltaUnhealthyApplicationsPercent")]
    pub r#max_delta_unhealthy_applications_percent: Box<Option<i32>>,
    /// Specifies the maximum tolerated percentage of delta unhealthy nodes that can have aggregated health states of error. If the current unhealthy nodes do not respect the percentage relative to the state at the beginning of the upgrade, the cluster is unhealthy. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "maxDeltaUnhealthyNodesPercent")]
    pub r#max_delta_unhealthy_nodes_percent: Box<Option<i32>>,
    /// Specifies the maximum tolerated percentage of upgrade domain delta unhealthy nodes that can have aggregated health state of error. If there is any upgrade domain where the current unhealthy nodes do not respect the percentage relative to the state at the beginning of the upgrade, the cluster is unhealthy. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "maxUpgradeDomainDeltaUnhealthyNodesPercent")]
    pub r#max_upgrade_domain_delta_unhealthy_nodes_percent: Box<Option<i32>>,
}
