#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterRestoreToPointInTime {
    /// Date and time in UTC format to restore the database cluster to. Conflicts with `use_latest_restorable_time`.
    #[builder(into, default)]
    #[serde(rename = "restoreToTime")]
    pub r#restore_to_time: Box<Option<String>>,
    /// Type of restore to be performed.
    /// Valid options are `full-copy` (default) and `copy-on-write`.
    #[builder(into, default)]
    #[serde(rename = "restoreType")]
    pub r#restore_type: Box<Option<String>>,
    /// Identifier of the source database cluster from which to restore. When restoring from a cluster in another AWS account, the identifier is the ARN of that cluster.
    #[builder(into, default)]
    #[serde(rename = "sourceClusterIdentifier")]
    pub r#source_cluster_identifier: Box<Option<String>>,
    /// Cluster resource ID of the source database cluster from which to restore. To be used for restoring a deleted cluster in the same account which still has a retained automatic backup available.
    #[builder(into, default)]
    #[serde(rename = "sourceClusterResourceId")]
    pub r#source_cluster_resource_id: Box<Option<String>>,
    /// Set to true to restore the database cluster to the latest restorable backup time. Defaults to false. Conflicts with `restore_to_time`.
    #[builder(into, default)]
    #[serde(rename = "useLatestRestorableTime")]
    pub r#use_latest_restorable_time: Box<Option<bool>>,
}
