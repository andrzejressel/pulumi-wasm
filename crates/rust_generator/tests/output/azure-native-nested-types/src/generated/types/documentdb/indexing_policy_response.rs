#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IndexingPolicyResponse {
    /// List of composite path list
    #[builder(into, default)]
    #[serde(rename = "compositeIndexes")]
    pub r#composite_indexes: Box<Option<Vec<Vec<super::super::types::documentdb::CompositePathResponse>>>>,
}
