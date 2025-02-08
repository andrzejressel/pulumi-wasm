#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FieldIndexConfigIndex {
    /// Indicates that this field supports operations on arrayValues. Only one of `order` and `arrayConfig` can
    /// be specified.
    /// Possible values are: `CONTAINS`.
    #[builder(into, default)]
    #[serde(rename = "arrayConfig")]
    pub r#array_config: Box<Option<String>>,
    /// Indicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=, !=.
    /// Only one of `order` and `arrayConfig` can be specified.
    /// Possible values are: `ASCENDING`, `DESCENDING`.
    #[builder(into, default)]
    #[serde(rename = "order")]
    pub r#order: Box<Option<String>>,
    /// The scope at which a query is run. Collection scoped queries require you specify
    /// the collection at query time. Collection group scope allows queries across all
    /// collections with the same id.
    /// Default value is `COLLECTION`.
    /// Possible values are: `COLLECTION`, `COLLECTION_GROUP`.
    #[builder(into, default)]
    #[serde(rename = "queryScope")]
    pub r#query_scope: Box<Option<String>>,
}
