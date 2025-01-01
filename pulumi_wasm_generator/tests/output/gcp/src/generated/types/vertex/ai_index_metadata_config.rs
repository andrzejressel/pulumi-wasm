#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiIndexMetadataConfig {
    /// The configuration with regard to the algorithms used for efficient search.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "algorithmConfig")]
    pub r#algorithm_config: Box<Option<super::super::types::vertex::AiIndexMetadataConfigAlgorithmConfig>>,
    /// The default number of neighbors to find via approximate search before exact reordering is
    /// performed. Exact reordering is a procedure where results returned by an
    /// approximate search algorithm are reordered via a more expensive distance computation.
    /// Required if tree-AH algorithm is used.
    #[builder(into, default)]
    #[serde(rename = "approximateNeighborsCount")]
    pub r#approximate_neighbors_count: Box<Option<i32>>,
    /// The number of dimensions of the input vectors.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<i32>,
    /// The distance measure used in nearest neighbor search. The value must be one of the followings:
    /// * SQUARED_L2_DISTANCE: Euclidean (L_2) Distance
    /// * L1_DISTANCE: Manhattan (L_1) Distance
    /// * COSINE_DISTANCE: Cosine Distance. Defined as 1 - cosine similarity.
    /// * DOT_PRODUCT_DISTANCE: Dot Product Distance. Defined as a negative of the dot product
    #[builder(into, default)]
    #[serde(rename = "distanceMeasureType")]
    pub r#distance_measure_type: Box<Option<String>>,
    /// Type of normalization to be carried out on each vector. The value must be one of the followings:
    /// * UNIT_L2_NORM: Unit L2 normalization type
    /// * NONE: No normalization type is specified.
    #[builder(into, default)]
    #[serde(rename = "featureNormType")]
    pub r#feature_norm_type: Box<Option<String>>,
    /// Index data is split into equal parts to be processed. These are called "shards".
    /// The shard size must be specified when creating an index. The value must be one of the followings:
    /// * SHARD_SIZE_SMALL: Small (2GB)
    /// * SHARD_SIZE_MEDIUM: Medium (20GB)
    /// * SHARD_SIZE_LARGE: Large (50GB)
    #[builder(into, default)]
    #[serde(rename = "shardSize")]
    pub r#shard_size: Box<Option<String>>,
}
