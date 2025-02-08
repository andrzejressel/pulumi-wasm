#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct IndexDocumentMetadataConfigurationUpdate {
    /// The name of the index field. Minimum length of 1. Maximum length of 30.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A block that provides manual tuning parameters to determine how the field affects the search results. Detailed below
    #[builder(into, default)]
    #[serde(rename = "relevance")]
    pub r#relevance: Box<Option<super::super::types::kendra::IndexDocumentMetadataConfigurationUpdateRelevance>>,
    /// A block that provides information about how the field is used during a search. Documented below. Detailed below
    #[builder(into, default)]
    #[serde(rename = "search")]
    pub r#search: Box<Option<super::super::types::kendra::IndexDocumentMetadataConfigurationUpdateSearch>>,
    /// The data type of the index field. Valid values are `STRING_VALUE`, `STRING_LIST_VALUE`, `LONG_VALUE`, `DATE_VALUE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
