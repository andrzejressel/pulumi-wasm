#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustTunnelCloudflaredConfigConfigIngressRuleOriginRequestAccess {
    /// Audience tags of the access rule.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "audTags")]
    pub r#aud_tags: Box<Option<Vec<String>>>,
    /// Whether the access rule is required.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    /// Name of the team to which the access rule applies.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "teamName")]
    pub r#team_name: Box<Option<String>>,
}
