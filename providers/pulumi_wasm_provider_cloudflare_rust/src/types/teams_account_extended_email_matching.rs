#[derive(serde::Serialize)]
pub struct TeamsAccountExtendedEmailMatching {
    /// Whether e-mails should be matched on all variants of user emails (with + or . modifiers) in Firewall policies.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
