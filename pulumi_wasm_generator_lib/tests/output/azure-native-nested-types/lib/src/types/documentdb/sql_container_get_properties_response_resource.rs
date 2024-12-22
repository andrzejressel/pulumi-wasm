#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SqlContainerGetPropertiesResponseResource {
    /// The configuration of the indexing policy. By default, the indexing is automatic for all document paths within the container
    #[builder(into, default)]
    #[serde(rename = "indexingPolicy")]
    pub r#indexing_policy: Box<Option<crate::types::documentdb::IndexingPolicyResponse>>,
}
