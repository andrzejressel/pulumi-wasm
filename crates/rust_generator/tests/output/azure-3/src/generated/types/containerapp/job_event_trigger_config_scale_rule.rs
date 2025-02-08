#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobEventTriggerConfigScaleRule {
    /// A `authentication` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "authentications")]
    pub r#authentications: Box<Option<Vec<super::super::types::containerapp::JobEventTriggerConfigScaleRuleAuthentication>>>,
    /// Type of the scale rule.
    #[builder(into)]
    #[serde(rename = "customRuleType")]
    pub r#custom_rule_type: Box<String>,
    /// Metadata properties to describe the scale rule.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<std::collections::HashMap<String, String>>,
    /// Name of the scale rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
