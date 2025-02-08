#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionSchemaSource {
    /// The schema name in the source.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The Doc AI processor type name.
    #[builder(into, default)]
    #[serde(rename = "processorType")]
    pub r#processor_type: Box<Option<String>>,
}
