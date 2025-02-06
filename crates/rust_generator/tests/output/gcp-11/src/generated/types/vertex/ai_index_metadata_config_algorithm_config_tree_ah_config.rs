#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiIndexMetadataConfigAlgorithmConfigTreeAhConfig {
    /// Number of embeddings on each leaf node. The default value is 1000 if not set.
    #[builder(into, default)]
    #[serde(rename = "leafNodeEmbeddingCount")]
    pub r#leaf_node_embedding_count: Box<Option<i32>>,
    /// The default percentage of leaf nodes that any query may be searched. Must be in
    /// range 1-100, inclusive. The default value is 10 (means 10%) if not set.
    #[builder(into, default)]
    #[serde(rename = "leafNodesToSearchPercent")]
    pub r#leaf_nodes_to_search_percent: Box<Option<i32>>,
}
