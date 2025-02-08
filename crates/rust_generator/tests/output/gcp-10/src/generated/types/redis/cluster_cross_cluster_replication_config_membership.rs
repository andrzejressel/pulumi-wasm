#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterCrossClusterReplicationConfigMembership {
    /// Details of the primary cluster that is used as the replication source for all the secondary clusters.
    #[builder(into, default)]
    #[serde(rename = "primaryClusters")]
    pub r#primary_clusters: Box<Option<Vec<super::super::types::redis::ClusterCrossClusterReplicationConfigMembershipPrimaryCluster>>>,
    /// List of secondary clusters that are replicating from the primary cluster.
    #[builder(into, default)]
    #[serde(rename = "secondaryClusters")]
    pub r#secondary_clusters: Box<Option<Vec<super::super::types::redis::ClusterCrossClusterReplicationConfigMembershipSecondaryCluster>>>,
}
