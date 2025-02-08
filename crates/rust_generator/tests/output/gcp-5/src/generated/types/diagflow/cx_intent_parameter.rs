#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CxIntentParameter {
    /// The entity type of the parameter.
    /// Format: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types.
    #[builder(into)]
    #[serde(rename = "entityType")]
    pub r#entity_type: Box<String>,
    /// The unique identifier of the parameter. This field is used by training phrases to annotate their parts.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Indicates whether the parameter represents a list of values.
    #[builder(into, default)]
    #[serde(rename = "isList")]
    pub r#is_list: Box<Option<bool>>,
    /// Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging.
    /// Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled.
    #[builder(into, default)]
    #[serde(rename = "redact")]
    pub r#redact: Box<Option<bool>>,
}
