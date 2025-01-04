#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationInlineConfiguration {
    /// Configuration of the condition used for the target document attribute or metadata field when ingesting documents into Amazon Kendra. See condition.
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationCondition>>,
    /// `TRUE` to delete content if the condition used for the target attribute is met.
    #[builder(into, default)]
    #[serde(rename = "documentContentDeletion")]
    pub r#document_content_deletion: Box<Option<bool>>,
    /// Configuration of the target document attribute or metadata field when ingesting documents into Amazon Kendra. You can also include a value. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "target")]
    pub r#target: Box<Option<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTarget>>,
}
