#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequestAccess {
    #[serde(rename = "audTags")]
    pub r#aud_tags: Box<Option<Vec<String>>>,
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    #[serde(rename = "teamName")]
    pub r#team_name: Box<Option<String>>,
}
