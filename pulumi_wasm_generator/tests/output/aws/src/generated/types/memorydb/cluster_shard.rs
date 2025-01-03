#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterShard {
    /// Name of the cluster. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Set of nodes in this shard.
    #[builder(into, default)]
    #[serde(rename = "nodes")]
    pub r#nodes: Box<Option<Vec<super::super::types::memorydb::ClusterShardNode>>>,
    /// Number of individual nodes in this shard.
    #[builder(into, default)]
    #[serde(rename = "numNodes")]
    pub r#num_nodes: Box<Option<i32>>,
    /// Keyspace for this shard. Example: `0-16383`.
    #[builder(into, default)]
    #[serde(rename = "slots")]
    pub r#slots: Box<Option<String>>,
}
