#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAgentAgentVersionsAgentVersionSummaryGuardrailConfiguration {
    /// Unique identifier of the guardrail.
    #[builder(into)]
    #[serde(rename = "guardrailIdentifier")]
    pub r#guardrail_identifier: Box<String>,
    /// Version of the guardrail.
    #[builder(into)]
    #[serde(rename = "guardrailVersion")]
    pub r#guardrail_version: Box<String>,
}
