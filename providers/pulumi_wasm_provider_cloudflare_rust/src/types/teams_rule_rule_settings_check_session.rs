#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsCheckSession {
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    #[serde(rename = "enforce")]
    pub r#enforce: Box<bool>,
}
