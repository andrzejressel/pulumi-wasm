#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequestAccess {
    /// Audience tags of the access rule.
    #[serde(rename = "audTags")]
    pub r#aud_tags: Box<Option<Vec<String>>>,
    /// Whether the access rule is required.
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    /// Name of the team to which the access rule applies.
    #[serde(rename = "teamName")]
    pub r#team_name: Box<Option<String>>,
}
