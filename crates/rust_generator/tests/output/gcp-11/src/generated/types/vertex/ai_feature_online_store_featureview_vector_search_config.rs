#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiFeatureOnlineStoreFeatureviewVectorSearchConfig {
    /// Configuration options for using brute force search, which simply implements the standard linear search in the database for each query. It is primarily meant for benchmarking and to generate the ground truth for approximate search.
    #[builder(into, default)]
    #[serde(rename = "bruteForceConfig")]
    pub r#brute_force_config: Box<Option<super::super::types::vertex::AiFeatureOnlineStoreFeatureviewVectorSearchConfigBruteForceConfig>>,
    /// Column of crowding. This column contains crowding attribute which is a constraint on a neighbor list produced by nearest neighbor search requiring that no more than some value k' of the k neighbors returned have the same value of crowdingAttribute.
    #[builder(into, default)]
    #[serde(rename = "crowdingColumn")]
    pub r#crowding_column: Box<Option<String>>,
    /// The distance measure used in nearest neighbor search.
    /// For details on allowed values, see the [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.featureOnlineStores.featureViews#DistanceMeasureType).
    /// Possible values are: `SQUARED_L2_DISTANCE`, `COSINE_DISTANCE`, `DOT_PRODUCT_DISTANCE`.
    #[builder(into, default)]
    #[serde(rename = "distanceMeasureType")]
    pub r#distance_measure_type: Box<Option<String>>,
    /// Column of embedding. This column contains the source data to create index for vector search.
    #[builder(into)]
    #[serde(rename = "embeddingColumn")]
    pub r#embedding_column: Box<String>,
    /// The number of dimensions of the input embedding.
    #[builder(into, default)]
    #[serde(rename = "embeddingDimension")]
    pub r#embedding_dimension: Box<Option<i32>>,
    /// Columns of features that are used to filter vector search results.
    #[builder(into, default)]
    #[serde(rename = "filterColumns")]
    pub r#filter_columns: Box<Option<Vec<String>>>,
    /// Configuration options for the tree-AH algorithm (Shallow tree + Asymmetric Hashing). Please refer to this paper for more details: https://arxiv.org/abs/1908.10396
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "treeAhConfig")]
    pub r#tree_ah_config: Box<Option<super::super::types::vertex::AiFeatureOnlineStoreFeatureviewVectorSearchConfigTreeAhConfig>>,
}
