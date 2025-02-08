#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceStateInfoUpdateInfo {
    /// (Output)
    /// Output only. Target number of replica nodes per shard for the instance.
    #[builder(into, default)]
    #[serde(rename = "targetReplicaCount")]
    pub r#target_replica_count: Box<Option<i32>>,
    /// (Output)
    /// Output only. Target number of shards for the instance.
    #[builder(into, default)]
    #[serde(rename = "targetShardCount")]
    pub r#target_shard_count: Box<Option<i32>>,
}
