#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GremlinGraphIndexPolicy {
    /// Indicates if the indexing policy is automatic. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "automatic")]
    pub r#automatic: Box<Option<bool>>,
    /// One or more `composite_index` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "compositeIndices")]
    pub r#composite_indices: Box<Option<Vec<super::super::types::cosmosdb::GremlinGraphIndexPolicyCompositeIndex>>>,
    /// List of paths to exclude from indexing. Required if `indexing_mode` is `Consistent` or `Lazy`.
    #[builder(into, default)]
    #[serde(rename = "excludedPaths")]
    pub r#excluded_paths: Box<Option<Vec<String>>>,
    /// List of paths to include in the indexing. Required if `indexing_mode` is `Consistent` or `Lazy`.
    #[builder(into, default)]
    #[serde(rename = "includedPaths")]
    pub r#included_paths: Box<Option<Vec<String>>>,
    /// Indicates the indexing mode. Possible values include: `Consistent`, `Lazy`, `None`.
    #[builder(into)]
    #[serde(rename = "indexingMode")]
    pub r#indexing_mode: Box<String>,
    /// One or more `spatial_index` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "spatialIndices")]
    pub r#spatial_indices: Box<Option<Vec<super::super::types::cosmosdb::GremlinGraphIndexPolicySpatialIndex>>>,
}
