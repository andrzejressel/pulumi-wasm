#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsRuleRuleSettingsCheckSession {
    /// Configure how fresh the session needs to be to be considered valid.
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// Enable session enforcement for this rule.
    #[serde(rename = "enforce")]
    pub r#enforce: Box<bool>,
}
