#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsUntrustedCert {
    /// Action to be taken when the SSL certificate of upstream is invalid. Available values: `pass_through`, `block`, `error`.
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
}
