#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterShard {
    /// Name of the cluster.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Set of nodes in this shard.
    #[builder(into)]
    #[serde(rename = "nodes")]
    pub r#nodes: Box<Vec<super::super::types::memorydb::GetClusterShardNode>>,
    /// Number of individual nodes in this shard.
    #[builder(into)]
    #[serde(rename = "numNodes")]
    pub r#num_nodes: Box<i32>,
    /// Keyspace for this shard. Example: `0-16383`.
    #[builder(into)]
    #[serde(rename = "slots")]
    pub r#slots: Box<String>,
}
