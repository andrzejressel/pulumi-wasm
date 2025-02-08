#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableSchemaDefinitionClusteringKey {
    /// The name of the clustering key column.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The order modifier. Valid values: `ASC`, `DESC`.
    #[builder(into)]
    #[serde(rename = "orderBy")]
    pub r#order_by: Box<String>,
}
