#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TeamsRuleRuleSettingsCheckSession {
    /// Configure how fresh the session needs to be to be considered valid.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// Enable session enforcement for this rule.
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: Box<bool>,
}
