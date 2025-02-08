#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SqlContainerGetPropertiesResponseResource {
    /// The configuration of the indexing policy. By default, the indexing is automatic for all document paths within the container
    #[builder(into, default)]
    #[serde(rename = "indexingPolicy")]
    pub r#indexing_policy: Box<Option<super::super::types::documentdb::IndexingPolicyResponse>>,
}
