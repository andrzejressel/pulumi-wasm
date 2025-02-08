#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GremlinGraphIndexPolicyCompositeIndexIndex {
    /// Order of the index. Possible values are `Ascending` or `Descending`.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Box<String>,
    /// Path for which the indexing behaviour applies to.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
