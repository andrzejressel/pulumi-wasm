#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsEgress {
    #[serde(rename = "ipv4")]
    pub r#ipv_4: Box<String>,
    #[serde(rename = "ipv4Fallback")]
    pub r#ipv_4_fallback: Box<Option<String>>,
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<String>,
}
