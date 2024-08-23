#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsL4Override {
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
