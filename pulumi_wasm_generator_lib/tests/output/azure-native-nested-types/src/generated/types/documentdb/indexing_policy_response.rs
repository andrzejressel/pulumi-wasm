#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct IndexingPolicyResponse {
    /// List of composite path list
    #[builder(into, default)]
    #[serde(rename = "compositeIndexes")]
    pub r#composite_indexes: Box<Option<Vec<Vec<super::super::types::documentdb::CompositePathResponse>>>>,
}
