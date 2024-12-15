#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
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
