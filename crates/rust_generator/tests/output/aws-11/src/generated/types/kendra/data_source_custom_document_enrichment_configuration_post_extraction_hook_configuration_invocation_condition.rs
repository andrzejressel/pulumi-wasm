#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfigurationInvocationCondition {
    /// The identifier of the document attribute used for the condition. For example, `_source_uri` could be an identifier for the attribute or metadata field that contains source URIs associated with the documents. Amazon Kendra currently does not support `_document_body` as an attribute key used for the condition.
    #[builder(into)]
    #[serde(rename = "conditionDocumentAttributeKey")]
    pub r#condition_document_attribute_key: Box<String>,
    /// The value used by the operator. For example, you can specify the value 'financial' for strings in the `_source_uri` field that partially match or contain this value. See condition_on_value.
    #[builder(into, default)]
    #[serde(rename = "conditionOnValue")]
    pub r#condition_on_value: Box<Option<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfigurationInvocationConditionConditionOnValue>>,
    /// The condition operator. For example, you can use `Contains` to partially match a string. Valid Values: `GreaterThan` | `GreaterThanOrEquals` | `LessThan` | `LessThanOrEquals` | `Equals` | `NotEquals` | `Contains` | `NotContains` | `Exists` | `NotExists` | `BeginsWith`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
}
