#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsL4Override {
    /// Override IP to forward traffic to.
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
    /// Override Port to forward traffic to.
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
