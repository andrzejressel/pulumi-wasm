#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterStateInfoUpdateInfo {
    /// Target number of replica nodes per shard.
    #[builder(into, default)]
    #[serde(rename = "targetReplicaCount")]
    pub r#target_replica_count: Box<Option<i32>>,
    /// Target number of shards for redis cluster.
    #[builder(into, default)]
    #[serde(rename = "targetShardCount")]
    pub r#target_shard_count: Box<Option<i32>>,
}
