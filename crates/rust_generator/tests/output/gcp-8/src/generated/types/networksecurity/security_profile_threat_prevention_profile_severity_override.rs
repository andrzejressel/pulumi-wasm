#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityProfileThreatPreventionProfileSeverityOverride {
    /// Threat action override.
    /// Possible values are: `ALERT`, `ALLOW`, `DEFAULT_ACTION`, `DENY`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Severity level to match.
    /// Possible values are: `CRITICAL`, `HIGH`, `INFORMATIONAL`, `LOW`, `MEDIUM`.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Box<String>,
}
