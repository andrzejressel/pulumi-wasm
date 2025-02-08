#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EntryBigqueryDateShardedSpec {
    /// (Output)
    /// The Data Catalog resource name of the dataset entry the current table belongs to, for example,
    /// projects/{project_id}/locations/{location}/entrygroups/{entryGroupId}/entries/{entryId}
    #[builder(into, default)]
    #[serde(rename = "dataset")]
    pub r#dataset: Box<Option<String>>,
    /// (Output)
    /// Total number of shards.
    #[builder(into, default)]
    #[serde(rename = "shardCount")]
    pub r#shard_count: Box<Option<i32>>,
    /// (Output)
    /// The table name prefix of the shards. The name of any given shard is [tablePrefix]YYYYMMDD,
    /// for example, for shard MyTable20180101, the tablePrefix is MyTable.
    #[builder(into, default)]
    #[serde(rename = "tablePrefix")]
    pub r#table_prefix: Box<Option<String>>,
}
