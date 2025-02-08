#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuardrailContentPolicyConfigFiltersConfig {
    /// Strength for filters.
    #[builder(into)]
    #[serde(rename = "inputStrength")]
    pub r#input_strength: Box<String>,
    /// Strength for filters.
    #[builder(into)]
    #[serde(rename = "outputStrength")]
    pub r#output_strength: Box<String>,
    /// Type of contextual grounding filter.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
